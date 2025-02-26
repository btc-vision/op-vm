use bytes::Bytes;
use chrono::Local;
use napi::bindgen_prelude::*;
use napi::bindgen_prelude::{Array, BigInt, Buffer, Undefined};
use napi::Env;
use napi::Error;
use napi::JsUnknown;
use std::panic::catch_unwind;
use std::sync::{Arc, Mutex, TryLockError};
use tokio::runtime::Runtime;
use wasmer::Value;

use crate::application::contract::ContractService;
use crate::domain::runner::{CustomEnv, WasmerRunner};
use crate::domain::vm::log_time_diff;
use crate::interfaces::napi::contract::JsContractParameter;
use crate::interfaces::napi::js_contract_manager::ContractManager;
use crate::interfaces::napi::runtime_pool::RuntimePool;
use crate::interfaces::{
    CallOtherContractExternalFunction, ConsoleLogExternalFunction,
    DeployFromAddressExternalFunction, EmitExternalFunction, InputsExternalFunction,
    OutputsExternalFunction, RevertDataResponse, StorageLoadExternalFunction,
    StorageStoreExternalFunction,
};

#[napi(object)]
pub struct CallResponse {
    #[napi(ts_type = "number[]")]
    pub result: Array,
    pub gas_used: BigInt,
}

pub struct JsContract {
    runner: Arc<Mutex<WasmerRunner>>,
    contract: Arc<Mutex<ContractService>>,
    runtime: Arc<Runtime>,
    runtime_pool: Arc<RuntimePool>,
}

impl JsContract {
    pub fn validate_bytecode(bytecode: Buffer, max_gas: BigInt) -> Result<bool> {
        let time = Local::now();
        let bytecode_vec = bytecode.to_vec();
        let max_gas = max_gas.get_u64().1;

        WasmerRunner::validate_bytecode(&bytecode_vec, max_gas)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

        log_time_diff(&time, "JsContract::validate");

        Ok(true)
    }

    pub fn from(params: JsContractParameter, manager: &ContractManager, id: u64) -> Result<Self> {
        catch_unwind(|| unsafe {
            let time = Local::now();

            let storage_load_tsfn = manager.storage_load_tsfn.clone();
            let storage_store_tsfn = manager.storage_store_tsfn.clone();
            let call_other_contract_tsfn = manager.call_other_contract_tsfn.clone();
            let deploy_from_address_tsfn = manager.deploy_from_address_tsfn.clone();
            let console_log_tsfn = manager.console_log_tsfn.clone();
            let emit_tsfn = manager.emit_tsfn.clone();
            let inputs_tsfn = manager.inputs_tsfn.clone();
            let outputs_tsfn = manager.outputs_tsfn.clone();

            // Create ExternalFunction instances with contract_id
            let storage_load_external = StorageLoadExternalFunction::new(storage_load_tsfn, id);
            let storage_store_external = StorageStoreExternalFunction::new(storage_store_tsfn, id);
            let call_other_contract_external =
                CallOtherContractExternalFunction::new(call_other_contract_tsfn, id);
            let deploy_from_address_external =
                DeployFromAddressExternalFunction::new(deploy_from_address_tsfn, id);
            let console_log_external = ConsoleLogExternalFunction::new(console_log_tsfn, id);
            let emit_external = EmitExternalFunction::new(emit_tsfn, id);
            let inputs_external = InputsExternalFunction::new(inputs_tsfn, id);
            let outputs_external = OutputsExternalFunction::new(outputs_tsfn, id);

            // Obtain a Runtime from the pool
            let runtime = manager.runtime_pool.get_runtime().ok_or_else(|| {
                Error::from_reason("No available runtimes in the pool".to_string())
            })?;

            //let runtime = Arc::new(Runtime::new()?);
            let custom_env: CustomEnv = CustomEnv::new(
                params.network.into(),
                storage_load_external,
                storage_store_external,
                call_other_contract_external,
                deploy_from_address_external,
                console_log_external,
                emit_external,
                inputs_external,
                outputs_external,
                runtime.clone(),
            )
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

            let runner: WasmerRunner;

            if let Some(bytecode) = params.bytecode {
                runner = WasmerRunner::from_bytecode(
                    &bytecode,
                    params.max_gas,
                    custom_env,
                    params.is_debug_mode,
                )
                .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
            } else if let Some(serialized) = params.serialized {
                runner = WasmerRunner::from_serialized(
                    serialized,
                    params.max_gas,
                    custom_env,
                    params.is_debug_mode,
                )
                .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
            } else {
                return Err(Error::from_reason("No bytecode or serialized data"));
            }

            let contract = JsContract::from_runner(
                runner,
                params.max_gas,
                runtime.clone(),
                manager.runtime_pool.clone(),
            )?; //, storage_load_tsfn, storage_store_tsfn, call_other_contract_tsfn, deploy_from_address_tsfn, console_log_tsfn
            log_time_diff(&time, "JsContract::from");

            Ok(contract)
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    pub fn call_sync(&self, func_name: &str, int_params: &[i32]) -> Result<Box<[Value]>> {
        // Convert the i32s to Wasmer `Value`
        let wasm_params: Vec<Value> = int_params.iter().map(|i| Value::I32(*i)).collect();

        // Lock the contract and call
        let mut svc = self
            .contract
            .lock()
            .map_err(|_| Error::from_reason("ContractService mutex poisoned"))?;

        let call_result = svc.call(func_name, &wasm_params);

        // Return the raw Values for later JS conversion
        match call_result {
            Ok(values) => Ok(values),
            Err(e) => Err(Error::from_reason(format!("{:?}", e))),
        }
    }

    fn from_runner(
        runner: WasmerRunner,
        max_gas: u64,
        runtime: Arc<Runtime>,
        runtime_pool: Arc<RuntimePool>,
    ) -> Result<Self> {
        let time = Local::now();

        let runner = Arc::new(Mutex::new(runner));
        let contract = ContractService::new(max_gas, runner.clone());

        log_time_diff(&time, "JsContract::from_runner");

        Ok(Self {
            runner,
            contract: Arc::new(Mutex::new(contract)),
            runtime,
            runtime_pool,
        })
    }

    pub fn serialize(&self) -> Result<Bytes> {
        let runner = self.runner.clone();
        let runner = runner.try_lock().map_err(|e| match e {
            TryLockError::Poisoned(_) => Error::from_reason("Runner mutex is poisoned".to_string()),
            TryLockError::WouldBlock => {
                Error::from_reason("Runner mutex is already locked".to_string())
            }
        })?;
        let serialized = runner
            .serialize()
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

        Ok(serialized)
    }

    pub fn read_memory(&self, offset: BigInt, length: BigInt) -> Result<Buffer> {
        let offset = offset.get_u64().1;
        let length = length.get_u64().1;
        let contract = self.contract.clone();

        let result = {
            let contract = contract.try_lock().map_err(|e| match e {
                TryLockError::Poisoned(_) => {
                    Error::from_reason("Contract mutex is poisoned".to_string())
                }
                TryLockError::WouldBlock => {
                    Error::from_reason("Contract mutex is already locked".to_string())
                }
            })?;
            contract.read_memory(offset, length)
        };

        let resp = result.map_err(|e| Error::from_reason(format!("{:?}", e)))?;

        Ok(Buffer::from(resp))
    }

    pub fn write_memory(&self, offset: BigInt, data: Buffer) -> Result<Undefined> {
        let data: Vec<u8> = data.into();
        let offset = offset.get_u64().1;
        let contract = self.contract.clone();

        let contract = contract.try_lock().map_err(|e| match e {
            TryLockError::Poisoned(_) => {
                Error::from_reason("Contract mutex is poisoned".to_string())
            }
            TryLockError::WouldBlock => {
                Error::from_reason("Contract mutex is already locked".to_string())
            }
        })?;
        contract
            .write_memory(offset, &data)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

        Ok(())
    }

    pub fn get_used_gas(&self) -> Result<BigInt> {
        let contract = self.contract.clone();
        let gas = {
            let mut contract = contract.try_lock().map_err(|e| match e {
                TryLockError::Poisoned(_) => {
                    Error::from_reason("Contract mutex is poisoned".to_string())
                }
                TryLockError::WouldBlock => {
                    Error::from_reason("Contract mutex is already locked".to_string())
                }
            })?;
            contract.get_used_gas()
        };

        Ok(BigInt::from(gas))
    }

    pub fn set_used_gas(&self, gas: BigInt) -> Result<()> {
        let gas = gas.get_u64().1;
        let contract = self.contract.clone();
        let mut contract = contract.try_lock().map_err(|e| match e {
            TryLockError::Poisoned(_) => {
                Error::from_reason("Contract mutex is poisoned".to_string())
            }
            TryLockError::WouldBlock => {
                Error::from_reason("Contract mutex is already locked".to_string())
            }
        })?;
        contract.set_used_gas(gas);

        Ok(())
    }

    pub fn get_remaining_gas(&self) -> Result<BigInt> {
        let contract = self.contract.clone();
        let gas = {
            let mut contract = contract.try_lock().map_err(|e| match e {
                TryLockError::Poisoned(_) => {
                    Error::from_reason("Contract mutex is poisoned".to_string())
                }
                TryLockError::WouldBlock => {
                    Error::from_reason("Contract mutex is already locked".to_string())
                }
            })?;
            contract.get_remaining_gas()
        };

        Ok(BigInt::from(gas))
    }

    pub fn set_remaining_gas(&self, gas: BigInt) -> Result<()> {
        let gas = gas.get_u64().1;
        let contract = self.contract.clone();
        let mut contract = contract.try_lock().map_err(|e| match e {
            TryLockError::Poisoned(_) => {
                Error::from_reason("Contract mutex is poisoned".to_string())
            }
            TryLockError::WouldBlock => {
                Error::from_reason("Contract mutex is already locked".to_string())
            }
        })?;
        contract.set_remaining_gas(gas);

        Ok(())
    }

    pub fn use_gas(&self, gas: BigInt) -> Result<()> {
        let gas = gas.get_u64().1;
        let contract = self.contract.clone();
        let mut contract = contract.try_lock().map_err(|e| match e {
            TryLockError::Poisoned(_) => {
                Error::from_reason("Contract mutex is poisoned".to_string())
            }
            TryLockError::WouldBlock => {
                Error::from_reason("Contract mutex is already locked".to_string())
            }
        })?;
        contract.use_gas(gas);

        Ok(())
    }

    pub fn write_buffer(&self, value: Buffer, id: i32, align: u32) -> Result<i64> {
        let value = value.to_vec();
        let contract = self.contract.clone();

        let result = {
            let mut contract = contract.try_lock().map_err(|e| match e {
                TryLockError::Poisoned(_) => {
                    Error::from_reason("Contract mutex is poisoned".to_string())
                }
                TryLockError::WouldBlock => {
                    Error::from_reason("Contract mutex is already locked".to_string())
                }
            })?;
            contract.write_buffer(&value, id, align)?
        };

        Ok(result)
    }

    pub fn get_revert_data(&self) -> Result<RevertDataResponse> {
        let contract = self.contract.clone();
        let result: Option<RevertDataResponse> = {
            let contract = contract.try_lock().map_err(|e| match e {
                TryLockError::Poisoned(_) => {
                    Error::from_reason("Contract mutex is poisoned".to_string())
                }
                TryLockError::WouldBlock => {
                    Error::from_reason("Contract mutex is already locked".to_string())
                }
            })?;
            contract.get_revert_data().map(|data| data.into())
        };

        result.ok_or(Error::from_reason("No revert data"))
    }

    /// Convert raw Wasmer `Value`s into a JS Array in the current Env
    pub fn convert_values_to_js_array(&self, env: &Env, values: Box<[Value]>) -> Result<Array> {
        Self::box_values_to_js_array(env, values)
    }
}

impl Drop for JsContract {
    fn drop(&mut self) {
        // Return the runtime to the pool
        if let Err(e) = self.runtime_pool.return_runtime(self.runtime.clone()) {
            eprintln!("Failed to return runtime to pool: {:?}", e);
        }
    }
}

impl JsContract {
    fn value_to_js(env: &Env, value: &Value) -> Result<JsUnknown> {
        match value {
            Value::I32(v) => {
                let js_value = env.create_int32(*v)?;
                let unknown = js_value.into_unknown();

                Ok(unknown)
            }
            Value::I64(v) => {
                let js_value = env.create_int64(*v)?;
                let unknown = js_value.into_unknown();

                Ok(unknown)
            }

            Value::F32(v) => {
                let js_value = env.create_double(*v as f64)?;
                let unknown = js_value.into_unknown();

                Ok(unknown)
            }

            Value::F64(v) => {
                let js_value = env.create_double(*v)?;
                let unknown = js_value.into_unknown();

                Ok(unknown)
            }

            Value::V128(v) => {
                let js_value = env.create_bigint_from_u128(*v)?;
                let unknown = js_value.into_unknown()?;

                Ok(unknown)
            }

            _ => Err(Error::from_reason("Unsupported value type")),
        }
    }

    pub fn box_values_to_js_array(env: &Env, values: Box<[Value]>) -> Result<Array> {
        let vals: Vec<Value> = values.clone().into_vec();
        let mut js_array = env.create_array(vals.len() as u32)?;

        for value in values.iter() {
            let js_value = JsContract::value_to_js(env, value)?;
            let _ = js_array.insert(js_value);
        }

        Ok(js_array)
    }
}
