use crate::application::contract::ContractService;
use crate::domain::runner::{CustomEnv, ExitData, WasmerRunner, MAX_PAGES};
use crate::domain::vm::log_time_diff;
use crate::interfaces::napi::contract::JsContractParameter;
use crate::interfaces::napi::environment_variables_request::EnvironmentVariablesRequest;
use crate::interfaces::napi::js_contract_manager::ContractManager;
use crate::interfaces::napi::runtime_pool::RuntimePool;
use crate::interfaces::{
    AccountTypeExternalFunction, BlockHashExternalFunction, CallOtherContractExternalFunction,
    ConsoleLogExternalFunction, DeployFromAddressExternalFunction, EmitExternalFunction,
    ExitDataResponse, InputsExternalFunction, OutputsExternalFunction, StorageLoadExternalFunction,
    StorageStoreExternalFunction,
};
use bytes::Bytes;
use chrono::Local;
use napi::bindgen_prelude::*;
use napi::bindgen_prelude::{Array, BigInt, Buffer};
use napi::Env;
use napi::Error;
use napi::JsUnknown;
use std::panic::catch_unwind;
use std::sync::{Arc, Mutex, TryLockError};
use tokio::runtime::Runtime;
use wasmer::Value;

pub struct JsContract {
    runner: Arc<Mutex<WasmerRunner>>,
    contract: Arc<Mutex<ContractService>>,
    runtime: Arc<Runtime>,
    runtime_pool: Arc<RuntimePool>,
}

impl JsContract {
    pub fn from(params: JsContractParameter, manager: &ContractManager, id: u64) -> Result<Self> {
        catch_unwind(|| {
            let time = Local::now();

            let storage_load_tsfn = manager.storage_load_tsfn.clone();
            let storage_store_tsfn = manager.storage_store_tsfn.clone();
            let call_other_contract_tsfn = manager.call_other_contract_tsfn.clone();
            let deploy_from_address_tsfn = manager.deploy_from_address_tsfn.clone();
            let console_log_tsfn = manager.console_log_tsfn.clone();
            let emit_tsfn = manager.emit_tsfn.clone();
            let inputs_tsfn = manager.inputs_tsfn.clone();
            let outputs_tsfn = manager.outputs_tsfn.clone();
            let account_type_tsfn = manager.account_type_tsfn.clone();
            let block_hash_tsfn = manager.block_hash_tsfn.clone();

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
            let account_type_external = AccountTypeExternalFunction::new(account_type_tsfn, id);
            let block_hash_external = BlockHashExternalFunction::new(block_hash_tsfn, id);

            // Obtain a Runtime from the pool
            let runtime = manager.runtime_pool.get_runtime().ok_or_else(|| {
                Error::from_reason("No available runtimes in the pool".to_string())
            })?;

            if params.memory_pages_used >= MAX_PAGES {
                return Err(Error::from_reason("No more memory pages available"));
            }

            let max_pages = MAX_PAGES - params.memory_pages_used;
            let return_proofs = params.return_proofs;

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
                account_type_external,
                block_hash_external,
                runtime.clone(),
                max_pages,
                return_proofs,
            )
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

            let runner: WasmerRunner;

            if let Some(bytecode) = params.bytecode {
                runner = WasmerRunner::from_bytecode(
                    &bytecode,
                    params.used_gas,
                    params.max_gas,
                    max_pages,
                    custom_env,
                    params.is_debug_mode,
                )
                .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
            } else if let Some(serialized) = params.serialized {
                unsafe {
                    runner = WasmerRunner::from_serialized(
                        serialized,
                        params.used_gas,
                        params.max_gas,
                        max_pages,
                        custom_env,
                        params.is_debug_mode,
                    )
                    .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
                }
            } else {
                return Err(Error::from_reason("No bytecode or serialized data"));
            }

            let contract = JsContract::from_runner(
                runner,
                params.max_gas,
                runtime.clone(),
                manager.runtime_pool.clone(),
            )?;

            log_time_diff(&time, "JsContract::from");

            Ok(contract)
        })
        .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    pub fn set_environment_variables(
        &self,
        environment_variables: EnvironmentVariablesRequest,
    ) -> Result<()> {
        let contract = self.contract.clone();

        let mut contract = contract.try_lock().map_err(|e| match e {
            TryLockError::Poisoned(_) => {
                Error::from_reason("Contract mutex is poisoned".to_string())
            }
            TryLockError::WouldBlock => {
                Error::from_reason("Contract mutex is already locked".to_string())
            }
        })?;
        contract
            .set_environment_variables(environment_variables.into())
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

        Ok(())
    }

    pub fn on_deploy(&self, calldata: Buffer) -> Result<ExitData> {
        // Lock the contract and call
        let mut contract = self
            .contract
            .lock()
            .map_err(|_| Error::from_reason("ContractService mutex poisoned"))?;

        let call_result = contract.on_deploy(&calldata);

        match call_result {
            Ok(values) => Ok(values),
            Err(e) => Err(Error::from_reason(format!("{:?}", e))),
        }
    }

    pub fn execute(&self, calldata: Buffer) -> Result<ExitData> {
        let time = Local::now();
        // Lock the contract and call
        let mut contract = self
            .contract
            .lock()
            .map_err(|_| Error::from_reason("ContractService mutex poisoned"))?;

        let call_result = contract.execute(&calldata);

        let result = match call_result {
            Ok(values) => Ok(values),

            // TODO: LEAVE LIKE THIS FOR NOW, SEE IF WE CAN STOP THE ERROR: ERROR: ERROR EVENTUALLY
            // FIXED THE MASSIVE ERROR OVERFLOW OF ERROR USING ROOT_CAUSE
            Err(e) => Err(Error::from_reason(e.root_cause().to_string())),
        };

        log_time_diff(&time, "JsContract::execute");

        result
    }

    pub fn call_export_by_name(
        &self,
        function_name: &str,
        int_params: &[i32],
    ) -> Result<Box<[Value]>> {
        // Convert the i32s to Wasmer `Value`
        let wasm_params: Vec<Value> = int_params.iter().map(|i| Value::I32(*i)).collect();

        // Lock the contract and call
        let mut svc = self
            .contract
            .lock()
            .map_err(|_| Error::from_reason("ContractService mutex poisoned"))?;

        let call_result = svc.call_export_by_name(function_name, &wasm_params);

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

    pub fn write_memory(&self, offset: BigInt, data: Buffer) -> Result<()> {
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

    pub fn get_exit_data(&self) -> Result<ExitDataResponse> {
        let contract = self.contract.clone();
        let result: ExitDataResponse = {
            let contract = contract.try_lock().map_err(|e| match e {
                TryLockError::Poisoned(_) => {
                    Error::from_reason("Contract mutex is poisoned".to_string())
                }
                TryLockError::WouldBlock => {
                    Error::from_reason("Contract mutex is already locked".to_string())
                }
            })?;
            contract.get_exit_data().into()
        };

        Ok(result)
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
