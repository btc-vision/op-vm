use std::panic::catch_unwind;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use chrono::Local;
use napi::bindgen_prelude::*;
use napi::{Error, JsNumber, Result as NapiResult};
use wasmer::Value;

use crate::application::contract::ContractService;
use crate::domain::runner::{CustomEnv, ThreadedWasmerRunner, WasmerRunner};
use crate::domain::tcp::SocketConnection;
use crate::domain::vm::log_time_diff;
use crate::interfaces::napi::contract::JsContractParameter;
use crate::interfaces::napi::js_contract_manager::ContractManager;
use crate::interfaces::{AbortDataResponse, ContractCallTask};

/// A struct representing one "contract instance" in JS
/// with a WASM runner, contract service, a runtime, etc.
/// In this updated version, we store a ThreadedWasmerRunner instead of Arc<Mutex<WasmerRunner>>.
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

            // Wrap our real_runner into a dedicated thread:
            let threaded_runner = ThreadedWasmerRunner::new(real_runner);

            // Build the contract
            let contract = JsContract::from_runner(threaded_runner, params.max_gas, socket_arc)?;

            log_time_diff(&time, "JsContract::from");
            Ok(contract)
        }
    }

    fn from_runner(
        runner: ThreadedWasmerRunner,
        max_gas: u64,
        socket: Arc<Mutex<SocketConnection>>,
    ) -> NapiResult<Self> {
        let time = Local::now();

        // Build a ContractService.
        // NOTE: Your existing ContractService may expect an Arc<Mutex<WasmerRunner>>.
        // You might adapt it to accept Arc<ThreadedWasmerRunner> or similar. Example:
        let runner_arc = Arc::new(runner);
        let contract_service = ContractService::new(max_gas, runner_arc.clone());

        log_time_diff(&time, "JsContract::from_runner");

        Ok(Self {
            runner: runner_arc,
            contract: Arc::new(Mutex::new(contract_service)),
            socket,
        })
    }

    pub fn serialize(&self) -> NapiResult<Bytes> {
        // Now we just ask the concurrency wrapper to serialize:
        let serialized = self
            .runner
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
                .map_err(|e| Error::from_reason(format!("Failed to get param: {:?}", e)))?;
            wasm_params.push(Value::I32(val));
        }

        // We delegate actual calling to a `ContractCallTask`, which presumably
        // calls the runner under the hood. That still works if `ContractService`
        // calls `ThreadedWasmerRunner`.
        let contract = self.contract.clone();
        let task = ContractCallTask::new(contract, &func_name, &wasm_params, time);
        Ok(AsyncTask::new(task))
    }

    pub fn read_memory(&self, offset: BigInt, length: BigInt) -> NapiResult<Buffer> {
        let offset = offset.get_u64().1;
        let length = length.get_u64().1;

        // If your `ContractService` calls into the runner, that’s fine. Otherwise,
        // we can call the runner directly here:
        let data = self
            .runner
            .read_memory(offset, length)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
        Ok(Buffer::from(data))
    }

    pub fn write_memory(&self, offset: BigInt, data: Buffer) -> NapiResult<Undefined> {
        let offset = offset.get_u64().1;
        let bytes = data.to_vec();

        self.runner
            .write_memory(offset, &bytes)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
        Ok(())
    }

    pub fn get_used_gas(&self) -> NapiResult<BigInt> {
        // This example calls into `ContractService`, which presumably tracks used gas
        // or delegates to the runner. Adjust as needed:
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
            let mut guard = self.socket.lock().unwrap();
            guard.close().unwrap();
        });
        // Additional cleanup if desired
    }
}

// Helper for converting Wasmer Values to JS. (Unchanged from your snippet.)
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

    /// Helper for converting a `Box<[Value]>` to a JS array
    pub fn box_values_to_js_array(env: &Env, values: Box<[Value]>) -> NapiResult<Array> {
        let slice: Vec<Value> = values.into();
        let mut js_array = env.create_array(slice.len() as u32)?;

        for val in slice {
            let js_val = Self::value_to_js(env, &val)?;
            js_array.insert(js_val)?;
        }
        Ok(js_array)
    }
}
