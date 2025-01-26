use std::panic::catch_unwind;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use chrono::Local;
use napi::bindgen_prelude::*;
use napi::{Error, JsNumber, JsObject, Result as NapiResult};
use wasmer::Value;

use crate::application::contract::ContractService;
use crate::domain::runner::{CustomEnv, ThreadedWasmerRunner, WasmerRunner};
use crate::domain::tcp::SocketConnection;
use crate::domain::vm::log_time_diff;
use crate::interfaces::napi::contract::JsContractParameter;
use crate::interfaces::napi::js_contract_manager::ContractManager;
use crate::interfaces::AbortDataResponse;

pub struct JsContract {
    runner: Arc<ThreadedWasmerRunner>,
    contract: Arc<Mutex<ContractService>>,
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
        unsafe {
            let time = Local::now();

            // Grab a socket connection
            let socket_arc = manager.get_connection().map_err(|e| {
                Error::from_reason(format!("Failed to get socket connection: {:?}", e))
            })?;

            {
                // Lock the socket to set an ID
                let mut sock = socket_arc.lock().map_err(|_e| {
                    Error::from_reason("Failed to lock socket connection (poisoned)")
                })?;
                sock.set_id(id);
            }

            // Build custom environment
            let custom_env: CustomEnv = CustomEnv::new(params.network.into(), socket_arc.clone())
                .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

            // Either from bytecode or from serialized
            let real_runner = if let Some(bytecode) = params.bytecode {
                WasmerRunner::from_bytecode(&bytecode, params.max_gas, custom_env)
            } else if let Some(serialized) = params.serialized {
                WasmerRunner::from_serialized(serialized, params.max_gas, custom_env)
            } else {
                return Err(Error::from_reason("No bytecode or serialized data"));
            }
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

            // Wrap our real_runner into a dedicated thread
            let threaded_runner = ThreadedWasmerRunner::new(real_runner);

            // Build the contract
            let runner_arc = Arc::new(threaded_runner);
            let contract_service = ContractService::new(params.max_gas, runner_arc.clone());

            let js_contract = JsContract {
                runner: runner_arc,
                contract: Arc::new(Mutex::new(contract_service)),
                socket: socket_arc,
            };

            log_time_diff(&time, "JsContract::from");

            Ok(js_contract)
        }
    }

    pub fn serialize(&self) -> NapiResult<Bytes> {
        let serialized = self
            .runner
            .serialize()
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
        Ok(serialized)
    }

    // ---------------------------------------------------------------------
    // Replaces `AsyncTask<ContractCallTask>`.
    // We'll return a real JS Promise that resolves to an object like:
    // {
    //   result: any[],    // from WASM call
    //   gasUsed: bigint, // how much gas was consumed
    // }
    // ---------------------------------------------------------------------
    pub fn call(&self, env: Env, func_name: String, params: Vec<JsNumber>) -> NapiResult<JsObject> {
        println!("-- JsContract::call()");
        // Convert to WASM values
        let mut wasm_params = Vec::with_capacity(params.len());
        for p in params {
            let val = p
                .get_int32()
                .map_err(|e| Error::from_reason(format!("Invalid param: {:?}", e)))?;
            wasm_params.push(Value::I32(val));
        }

        // Clone the Arc references we need in both the future AND the final callback
        let contract_ref_in_future = self.contract.clone();
        let contract_ref_in_callback = self.contract.clone();

        let func_name_ref = func_name.clone();
        let wasm_params_ref = wasm_params.clone();

        println!("-- JsContract::call() creating contract call task");

        // The async block that performs the call in a background tokio task
        let fut = async move {
            // 1. Re-entrancy check:
            {
                let mut svc = contract_ref_in_future
                    .lock()
                    .map_err(|_| Error::from_reason("ContractService mutex poisoned"))?;
                if svc.is_executing() {
                    return Err(Error::from_reason("Re-entrancy error"));
                }
                svc.set_executing(true);
            }

            // 2. Actually call the WASM:
            let call_result = {
                let mut svc = contract_ref_in_future
                    .lock()
                    .map_err(|_| Error::from_reason("ContractService mutex poisoned"))?;
                svc.call(&func_name_ref, &wasm_params_ref)
            };

            // 3. Unlock & clear re-entrancy:
            {
                let mut svc = contract_ref_in_future
                    .lock()
                    .map_err(|_| Error::from_reason("ContractService mutex poisoned"))?;
                svc.set_executing(false);
            }

            // 4. Convert `anyhow::Result<Box<[Value]>>` -> `NapiResult<Box<[Value]>>`
            match call_result {
                Ok(values) => Ok(values),
                Err(e) => Err(Error::from_reason(format!("{:?}", e))),
            }
        };

        println!(
            "JsContract::call() calling contract function: {}",
            func_name
        );

        // The final callback that runs once the future is resolved
        env.execute_tokio_future(fut, move |&mut env, wasm_values| {
            // This callback runs in Node's main thread after the future completes.
            // Convert the `Box<[Value]>` into a JS array
            let js_array = Self::box_values_to_js_array(&env, wasm_values)?;

            // Retrieve gas used:
            let gas_used = {
                let mut svc = contract_ref_in_callback
                    .lock()
                    .map_err(|_| Error::from_reason("ContractService mutex poisoned"))?;
                svc.get_used_gas()
            };

            // Build a JS object { result: Value[], gasUsed: bigint }
            let mut result_obj = env.create_object()?;
            result_obj.set("result", js_array)?;
            result_obj.set("gasUsed", BigInt::from(gas_used))?;

            Ok(result_obj)
        })
    }

    // ---------------------------------------------------------------------
    // Other synchronous helpers:
    // ---------------------------------------------------------------------
    pub fn read_memory(&self, offset: BigInt, length: BigInt) -> NapiResult<Buffer> {
        let offset = offset.get_u64().1;
        let length = length.get_u64().1;
        let data = self
            .runner
            .read_memory(offset, length)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
        Ok(Buffer::from(data))
    }

    pub fn write_memory(&self, offset: BigInt, data: Buffer) -> NapiResult<()> {
        let offset = offset.get_u64().1;
        let bytes = data.to_vec();

        self.runner
            .write_memory(offset, &bytes)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
        Ok(())
    }

    pub fn get_used_gas(&self) -> NapiResult<BigInt> {
        let gas = {
            let mut c = self.contract.lock().map_err(|_e| {
                Error::from_reason("Failed to lock ContractService (poisoned)".to_string())
            })?;
            c.get_used_gas()
        };
        Ok(BigInt::from(gas))
    }

    pub fn set_used_gas(&self, gas: BigInt) -> NapiResult<()> {
        let gas = gas.get_u64().1;
        let mut c = self.contract.lock().map_err(|_e| {
            Error::from_reason("Failed to lock ContractService (poisoned)".to_string())
        })?;
        c.set_used_gas(gas);
        Ok(())
    }

    pub fn get_remaining_gas(&self) -> NapiResult<BigInt> {
        let gas = self.runner.get_remaining_gas();
        Ok(BigInt::from(gas))
    }

    pub fn set_remaining_gas(&self, gas: BigInt) -> NapiResult<()> {
        let gas = gas.get_u64().1;
        self.runner.set_remaining_gas(gas);
        Ok(())
    }

    pub fn use_gas(&self, gas: BigInt) -> NapiResult<()> {
        let gas = gas.get_u64().1;
        self.runner.use_gas(gas);
        Ok(())
    }

    pub fn write_buffer(&self, value: Buffer, id: i32, align: u32) -> NapiResult<i64> {
        let data = value.to_vec();
        let result = self
            .runner
            .write_buffer(&data, id, align)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
        Ok(result)
    }

    pub fn get_abort_data(&self) -> NapiResult<AbortDataResponse> {
        let maybe = self.runner.get_abort_data().map(|d| d.into());
        maybe.ok_or_else(|| Error::from_reason("No abort data".to_string()))
    }
}

impl Drop for JsContract {
    fn drop(&mut self) {
        let _ = catch_unwind(|| {
            if let Ok(guard) = self.socket.lock() {
                let _ = guard.close();
            }
        });
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
                Ok(js_big.into_unknown()?)
            }
            _ => Err(Error::from_reason("Unsupported value type".to_string())),
        }
    }

    pub fn box_values_to_js_array(env: &Env, values: Box<[Value]>) -> NapiResult<Array> {
        let slice: Vec<Value> = values.into();
        let mut arr = env.create_array(slice.len() as u32)?;
        for val in slice {
            let js_val = Self::value_to_js(env, &val)?;
            arr.insert(js_val)?;
        }
        Ok(arr)
    }
}
