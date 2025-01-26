use std::panic::catch_unwind;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use chrono::Local;
use napi::bindgen_prelude::*;
use napi::{Error, JsNumber, Result as NapiResult};
use wasmer::Value;

use crate::application::contract::ContractService;
use crate::domain::runner::{CustomEnv, WasmerRunner};
use crate::domain::tcp::SocketConnection;
use crate::domain::vm::log_time_diff;
use crate::interfaces::napi::contract::JsContractParameter;
use crate::interfaces::napi::js_contract_manager::ContractManager;
use crate::interfaces::{AbortDataResponse, ContractCallTask};

/// A struct representing one "contract instance" in JS
/// with a WASM runner, contract service, a runtime, etc.
pub struct JsContract {
    runner: Arc<Mutex<WasmerRunner>>,
    contract: Arc<Mutex<ContractService>>,
    //runtime: Arc<Runtime>,
    socket: Arc<Mutex<SocketConnection>>,
}

impl JsContract {
    pub fn validate_bytecode(bytecode: Buffer, max_gas: BigInt) -> NapiResult<bool> {
        let time = Local::now();
        let bytecode_vec = bytecode.to_vec();
        let max_gas = max_gas.get_u64().1;

        WasmerRunner::validate_bytecode(&bytecode_vec, max_gas)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

        log_time_diff(&time, "JsContract::validate");

        Ok(true)
    }

    pub fn from(
        params: JsContractParameter,
        manager: &ContractManager,
        id: u64,
    ) -> NapiResult<Self> {
        // We catch_unwind for safety around any potential panic
        //catch_unwind(|| unsafe {
        unsafe {
            let time = Local::now();

            // Obtain a runtime from the manager's pool
            //let runtime = manager.get_runtime().ok_or_else(|| {
            //    Error::from_reason("No available runtimes in the pool".to_string())
            //})?;

            // Grab a socket connection (Arc<Mutex<SocketConnection>>)
            let socket_arc = manager.get_connection().map_err(|e| {
                Error::from_reason(format!("Failed to get socket connection: {:?}", e))
            })?;

            // Lock the socket to call `set_id`
            {
                let mut sock = socket_arc.lock().map_err(|_e| {
                    Error::from_reason("Failed to lock socket connection (poisoned)")
                })?;
                sock.set_id(id);
            }

            let custom_env: CustomEnv = CustomEnv::new(params.network.into(), socket_arc.clone())
                .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

            // Either from bytecode or from serialized
            let runner = if let Some(bytecode) = params.bytecode {
                WasmerRunner::from_bytecode(&bytecode, params.max_gas, custom_env)
            } else if let Some(serialized) = params.serialized {
                WasmerRunner::from_serialized(serialized, params.max_gas, custom_env)
            } else {
                return Err(Error::from_reason("No bytecode or serialized data"));
            }
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;


            let contract = JsContract::from_runner(
                runner,
                params.max_gas,
                //runtime.clone(),
                socket_arc.clone(),
            )?;

            log_time_diff(&time, "JsContract::from");
            Ok(contract)
        }
        //})
        //.unwrap_or_else(|e| Err(Error::from_reason(format!("Panic: {:?}", e))))
    }

    fn from_runner(
        runner: WasmerRunner,
        max_gas: u64,
        //runtime: Arc<Runtime>,
        socket: Arc<Mutex<SocketConnection>>,
    ) -> NapiResult<Self> {
        let time = Local::now();
        let runner = Arc::new(Mutex::new(runner));
        let contract = ContractService::new(max_gas, runner.clone());

        log_time_diff(&time, "JsContract::from_runner");

        Ok(Self {
            runner,
            contract: Arc::new(Mutex::new(contract)),
            //runtime,
            socket,
        })
    }

    pub fn serialize(&self) -> NapiResult<Bytes> {
        let runner = self.runner.clone();
        let runner = runner.try_lock().map_err(|e| match e {
            std::sync::TryLockError::Poisoned(_) => {
                Error::from_reason("Runner mutex is poisoned".to_string())
            }
            std::sync::TryLockError::WouldBlock => {
                Error::from_reason("Runner mutex is already locked".to_string())
            }
        })?;

        let serialized = runner
            .serialize()
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
        Ok(serialized)
    }

    /// Return an `AsyncTask<ContractCallTask>` so you can do `Promise<CallResponse>` in JS.
    pub fn call(
        &self,
        func_name: String,
        params: Vec<JsNumber>,
    ) -> NapiResult<AsyncTask<ContractCallTask>> {
        let time = Local::now();
        let mut wasm_params = Vec::new();
        for param in params {
            let val = param
                .get_int32()
                .map_err(|e| Error::from_reason(format!("Failed to get param value: {:?}", e)))?;
            wasm_params.push(Value::I32(val));
        }
        let contract = self.contract.clone();
        let task = ContractCallTask::new(contract, &func_name, &wasm_params, time);
        Ok(AsyncTask::new(task))
    }

    pub fn read_memory(&self, offset: BigInt, length: BigInt) -> NapiResult<Buffer> {
        let offset = offset.get_u64().1;
        let length = length.get_u64().1;
        let contract = self.contract.clone();

        let data = {
            let contract = contract.try_lock().map_err(|e| match e {
                std::sync::TryLockError::Poisoned(_) => {
                    Error::from_reason("Contract mutex is poisoned".to_string())
                }
                std::sync::TryLockError::WouldBlock => {
                    Error::from_reason("Contract mutex is locked".to_string())
                }
            })?;
            contract.read_memory(offset, length)
        }
        .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

        Ok(Buffer::from(data))
    }

    pub fn write_memory(&self, offset: BigInt, data: Buffer) -> NapiResult<Undefined> {
        let offset = offset.get_u64().1;
        let bytes: Vec<u8> = data.to_vec();
        let contract = self.contract.clone();

        {
            let contract = contract.try_lock().map_err(|e| match e {
                std::sync::TryLockError::Poisoned(_) => {
                    Error::from_reason("Contract mutex is poisoned".to_string())
                }
                std::sync::TryLockError::WouldBlock => {
                    Error::from_reason("Contract mutex is locked".to_string())
                }
            })?;
            contract
                .write_memory(offset, &bytes)
                .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
        }
        Ok(())
    }

    pub fn get_used_gas(&self) -> NapiResult<BigInt> {
        let contract = self.contract.clone();
        let gas = {
            let mut c = contract.try_lock().map_err(|e| match e {
                std::sync::TryLockError::Poisoned(_) => {
                    Error::from_reason("poisoned contract mutex".to_string())
                }
                std::sync::TryLockError::WouldBlock => {
                    Error::from_reason("contract is locked".to_string())
                }
            })?;
            c.get_used_gas()
        };
        Ok(BigInt::from(gas))
    }

    pub fn set_used_gas(&self, gas: BigInt) -> NapiResult<()> {
        let gas = gas.get_u64().1;
        let contract = self.contract.clone();
        let mut c = contract.try_lock().map_err(|e| match e {
            std::sync::TryLockError::Poisoned(_) => {
                Error::from_reason("poisoned contract mutex".to_string())
            }
            std::sync::TryLockError::WouldBlock => {
                Error::from_reason("contract is locked".to_string())
            }
        })?;
        c.set_used_gas(gas);
        Ok(())
    }

    pub fn get_remaining_gas(&self) -> NapiResult<BigInt> {
        let contract = self.contract.clone();
        let gas = {
            let mut c = contract.try_lock().map_err(|e| match e {
                std::sync::TryLockError::Poisoned(_) => {
                    Error::from_reason("poisoned contract".to_string())
                }
                std::sync::TryLockError::WouldBlock => {
                    Error::from_reason("already locked".to_string())
                }
            })?;
            c.get_remaining_gas()
        };
        Ok(BigInt::from(gas))
    }

    pub fn set_remaining_gas(&self, gas: BigInt) -> NapiResult<()> {
        let gas = gas.get_u64().1;
        let contract = self.contract.clone();
        let mut c = contract.try_lock().map_err(|e| match e {
            std::sync::TryLockError::Poisoned(_) => Error::from_reason("poisoned".to_string()),
            std::sync::TryLockError::WouldBlock => Error::from_reason("locked".to_string()),
        })?;
        c.set_remaining_gas(gas);
        Ok(())
    }

    pub fn use_gas(&self, gas: BigInt) -> NapiResult<()> {
        let gas = gas.get_u64().1;
        let contract = self.contract.clone();
        let mut c = contract.try_lock().map_err(|e| match e {
            std::sync::TryLockError::Poisoned(_) => Error::from_reason("poisoned".to_string()),
            std::sync::TryLockError::WouldBlock => Error::from_reason("locked".to_string()),
        })?;
        c.use_gas(gas);
        Ok(())
    }

    pub fn write_buffer(&self, value: Buffer, id: i32, align: u32) -> NapiResult<i64> {
        let data = value.to_vec();
        let contract = self.contract.clone();

        let result = {
            let mut c = contract.try_lock().map_err(|e| match e {
                std::sync::TryLockError::Poisoned(_) => Error::from_reason("poisoned".to_string()),
                std::sync::TryLockError::WouldBlock => Error::from_reason("locked".to_string()),
            })?;
            c.write_buffer(&data, id, align)?
        };
        Ok(result)
    }

    pub fn get_abort_data(&self) -> NapiResult<AbortDataResponse> {
        let c = self.contract.clone();
        let maybe = {
            let c = c.try_lock().map_err(|e| match e {
                std::sync::TryLockError::Poisoned(_) => Error::from_reason("poisoned".to_string()),
                std::sync::TryLockError::WouldBlock => Error::from_reason("locked".to_string()),
            })?;
            c.get_abort_data().map(|data| data.into())
        };
        maybe.ok_or_else(|| Error::from_reason("No abort data".to_string()))
    }
}

impl Drop for JsContract {
    fn drop(&mut self) {
        // Return the runtime to the pool
        //if let Err(e) = self.manager.return_runtime(self.runtime.clone()) {
        //    eprintln!("Failed to return runtime: {:?}", e);
        //}

        let _ = catch_unwind(|| {
            self.socket
                .lock()
                .unwrap_or_else(|e| {
                    eprintln!("Failed to lock socket connection: {:?}", e);
                    panic!("Failed to lock socket connection");
                })
                .close()
                .unwrap();
        });

        ()

        // Return the socket to the pool
        //if let Err(e) = self.manager.return_connection(self.socket.clone()) {
        //    eprintln!("Failed to return socket connection: {:?}", e);
        //}
    }
}

impl JsContract {
    fn value_to_js(env: &Env, value: &Value) -> NapiResult<Unknown> {
        match value {
            Value::I32(v) => {
                let js = env.create_int32(*v)?;
                Ok(js.into_unknown())
            }
            Value::I64(v) => {
                let js = env.create_int64(*v)?;
                Ok(js.into_unknown())
            }
            Value::F32(v) => {
                let js = env.create_double(*v as f64)?;
                Ok(js.into_unknown())
            }
            Value::F64(v) => {
                let js = env.create_double(*v)?;
                Ok(js.into_unknown())
            }
            Value::V128(v) => {
                let js_big = env.create_bigint_from_u128(*v)?;
                let unknown = js_big.into_unknown()?;
                Ok(unknown)
            }
            _ => Err(Error::from_reason("Unsupported value type".to_string())),
        }
    }

    /// Helper for converting a `Box<[Value]>` to a JS array
    pub fn box_values_to_js_array(env: &Env, values: Box<[Value]>) -> NapiResult<Array> {
        let slice: Vec<Value> = values.into_vec();
        let mut js_array = env.create_array(slice.len() as u32)?;

        for val in slice.into_iter() {
            let js_val = Self::value_to_js(env, &val)?;
            js_array.insert(js_val)?;
        }
        Ok(js_array)
    }
}
