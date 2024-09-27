use bytes::Bytes;
use chrono::Local;
use napi::bindgen_prelude::*;
use napi::bindgen_prelude::{Array, BigInt, Buffer, Undefined};
use napi::Env;
use napi::Error;
use napi::JsNumber;
use napi::JsUnknown;
use std::panic::catch_unwind;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use wasmer::Value;

use crate::application::contract::ContractService;
use crate::domain::runner::{CustomEnv, WasmerRunner};
use crate::domain::vm::log_time_diff;
use crate::interfaces::napi::contract::JsContractParameter;
use crate::interfaces::napi::js_contract_manager::ContractManager;
use crate::interfaces::napi::runtime_pool::RuntimePool;
use crate::interfaces::{
    AbortDataResponse, CallOtherContractExternalFunction, ConsoleLogExternalFunction,
    ContractCallTask, DeployFromAddressExternalFunction, FlushEventsExternalFunction,
    StorageLoadExternalFunction, StorageStoreExternalFunction,
};
/**/

pub struct JsContract {
    //pending_calls: Arc<AtomicUsize>,
    //is_destroyed: Arc<AtomicBool>,
    runner: Arc<Mutex<WasmerRunner>>,
    contract: Arc<Mutex<ContractService>>,
    runtime: Arc<Runtime>,
    runtime_pool: Arc<RuntimePool>,
}

impl JsContract {
    pub fn validate_bytecode(bytecode: Buffer, max_gas: BigInt) -> Result<bool> {
        catch_unwind(|| {
            let time = Local::now();
            let bytecode_vec = bytecode.to_vec();
            let max_gas = max_gas.get_u64().1;

            let is_valid = WasmerRunner::validate_bytecode(&bytecode_vec, max_gas)
                .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

            log_time_diff(&time, "JsContract::validate");

            Ok(is_valid)
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    pub fn from(params: JsContractParameter, manager: &ContractManager, id: u64) -> Result<Self> {
        catch_unwind(|| unsafe {
            let time = Local::now();

            let storage_load_tsfn = manager.storage_load_tsfn.clone();
            let storage_store_tsfn = manager.storage_store_tsfn.clone();
            let call_other_contract_tsfn = manager.call_other_contract_tsfn.clone();
            let deploy_from_address_tsfn = manager.deploy_from_address_tsfn.clone();
            let console_log_tsfn = manager.console_log_tsfn.clone();
            let flush_events_tsfn = manager.flush_events_tsfn.clone();

            // Create ExternalFunction instances with contract_id
            let storage_load_external = StorageLoadExternalFunction::new(storage_load_tsfn, id);
            let storage_store_external = StorageStoreExternalFunction::new(storage_store_tsfn, id);
            let call_other_contract_external =
                CallOtherContractExternalFunction::new(call_other_contract_tsfn, id);
            let deploy_from_address_external =
                DeployFromAddressExternalFunction::new(deploy_from_address_tsfn, id);
            let console_log_external = ConsoleLogExternalFunction::new(console_log_tsfn, id);
            let flush_events_external = FlushEventsExternalFunction::new(flush_events_tsfn, id);

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
                flush_events_external,
                runtime.clone(),
            )
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

            let runner: WasmerRunner;

            if let Some(bytecode) = params.bytecode {
                runner = WasmerRunner::from_bytecode(&bytecode, params.max_gas, custom_env)
                    .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
            } else if let Some(serialized) = params.serialized {
                runner = WasmerRunner::from_serialized(serialized, params.max_gas, custom_env)
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
        catch_unwind(|| {
            let runner = self.runner.clone();
            let runner = runner
                .lock()
                .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
            let serialized = runner
                .serialize()
                .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

            Ok(serialized)
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    pub fn call(
        &self,
        func_name: String,
        params: Vec<JsNumber>,
    ) -> Result<AsyncTask<ContractCallTask>> {
        catch_unwind(|| {
            let time = Local::now();
            let mut wasm_params = Vec::new();
            let length = params.len();

            for i in 0..length {
                let param = params.get(i).expect("Failed to get param");
                let param_value = param.get_int32();

                if param_value.is_err() {
                    return Err(Error::from_reason(format!(
                        "{:?}",
                        param_value.unwrap_err()
                    )));
                }

                wasm_params.push(Value::I32(param_value.unwrap()));
            }

            let contract = self.contract.clone();
            let result = AsyncTask::new(ContractCallTask::new(
                contract,
                &func_name,
                &wasm_params,
                time,
            ));

            Ok(result)
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    pub fn read_memory(&self, offset: BigInt, length: BigInt) -> Result<Buffer> {
        catch_unwind(|| {
            let offset = offset.get_u64().1;
            let length = length.get_u64().1;
            let contract = self.contract.clone();

            let result = {
                let contract = contract.lock().unwrap();
                contract.read_memory(offset, length)
            };

            let resp = result.unwrap();

            Ok(Buffer::from(resp))
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    pub fn write_memory(&self, offset: BigInt, data: Buffer) -> Result<Undefined> {
        catch_unwind(|| {
            let data: Vec<u8> = data.into();
            let offset = offset.get_u64().1;
            let contract = self.contract.clone();

            let contract = contract.lock().unwrap();
            contract.write_memory(offset, &data).unwrap();

            Ok(())
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    pub fn get_used_gas(&self) -> Result<BigInt> {
        catch_unwind(|| {
            let contract = self.contract.clone();
            let gas = {
                let mut contract = contract.lock().unwrap();
                contract.get_used_gas()
            };

            Ok(BigInt::from(gas))
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    pub fn set_used_gas(&self, gas: BigInt) -> Result<()> {
        catch_unwind(|| {
            let gas = gas.get_u64().1;
            let contract = self.contract.clone();
            let mut contract = contract.lock().unwrap();
            contract.set_used_gas(gas);

            Ok(())
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    pub fn get_remaining_gas(&self) -> Result<BigInt> {
        catch_unwind(|| {
            let contract = self.contract.clone();
            let gas = {
                let mut contract = contract.lock().unwrap();
                contract.get_remaining_gas()
            };

            Ok(BigInt::from(gas))
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    pub fn set_remaining_gas(&self, gas: BigInt) -> Result<()> {
        catch_unwind(|| {
            let gas = gas.get_u64().1;
            let contract = self.contract.clone();
            {
                let mut contract = contract.lock().unwrap();
                contract.set_remaining_gas(gas);
            }

            Ok(())
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    pub fn use_gas(&self, gas: BigInt) -> Result<()> {
        catch_unwind(|| {
            let gas = gas.get_u64().1;
            let contract = self.contract.clone();
            {
                let mut contract = contract.lock().unwrap();
                contract.use_gas(gas);
            }

            Ok(())
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    pub fn write_buffer(&self, value: Buffer, id: i32, align: u32) -> Result<i64> {
        catch_unwind(|| {
            let value = value.to_vec();
            let contract = self.contract.clone();

            let result = {
                let mut contract = contract.lock().unwrap();
                contract.write_buffer(&value, id, align)?
            };

            Ok(result)
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    pub fn get_abort_data(&self) -> Result<AbortDataResponse> {
        catch_unwind(|| {
            let contract = self.contract.clone();
            let result: Option<AbortDataResponse> = {
                let contract = contract.lock().unwrap();
                contract.get_abort_data().map(|data| data.into())
            };

            result.ok_or(Error::from_reason("No abort data")).into()
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }
}

impl Drop for JsContract {
    fn drop(&mut self) {
        //println!("Dropping JsContract");

        // Return the runtime to the pool
        self.runtime_pool.return_runtime(self.runtime.clone());
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
