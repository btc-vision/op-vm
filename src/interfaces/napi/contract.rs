use crate::application::contract::ContractService;
use crate::domain::runner::{CustomEnv, ExitData, WasmerRunner, MAX_PAGES};
use crate::domain::vm::log_time_diff;
use crate::interfaces::napi::contract_manager::ContractManager;
use crate::interfaces::napi::environment_variables_request::EnvironmentVariablesRequest;
use crate::interfaces::napi::runtime_pool::RuntimePool;
use crate::interfaces::{
    AccountTypeExternalFunction, BlockHashExternalFunction, CallOtherContractExternalFunction,
    ConsoleLogExternalFunction, DeployFromAddressExternalFunction, EmitExternalFunction,
    InputsExternalFunction, OutputsExternalFunction, StorageLoadExternalFunction,
    StorageStoreExternalFunction,
};
use anyhow::anyhow;
use bytes::Bytes;
use chrono::Local;
use neon::prelude::*;

use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use wasmer::Value;

use super::bitcoin_network_request::BitcoinNetworkRequest;
use super::GenericExternalFunction;

pub struct ContractParameter {
    pub(crate) bytecode: Option<Vec<u8>>,
    pub(crate) serialized: Option<Bytes>,
    pub(crate) used_gas: u64,
    pub(crate) max_gas: u64,
    pub(crate) memory_pages_used: u32,
    pub(crate) network: BitcoinNetworkRequest,
    pub(crate) is_debug_mode: bool,
}

pub struct Contract {
    runner: Arc<Mutex<WasmerRunner>>,
    contract: Arc<Mutex<ContractService>>,
    runtime: Arc<Runtime>,
    runtime_pool: Arc<RuntimePool>,
}

impl Contract {
    pub fn from(
        cx: &mut FunctionContext,
        params: ContractParameter,
        manager: &ContractManager,
        id: u64,
    ) -> NeonResult<Self> {
        let time = Local::now();

        // Create ExternalFunction instances with contract_id
        let storage_load_external = StorageLoadExternalFunction::new(
            manager.storage_load_js_function.clone(),
            cx.channel(),
            id,
        );
        let storage_store_external = StorageStoreExternalFunction::new(
            manager.storage_store_js_function.clone(),
            cx.channel(),
            id,
        );

        let call_other_contract_external = CallOtherContractExternalFunction::new(
            manager.call_other_contract_js_function.clone(),
            cx.channel(),
            id,
        );
        let deploy_from_address_external = DeployFromAddressExternalFunction::new(
            manager.deploy_from_address_js_function.clone(),
            cx.channel(),
            id,
        );
        let console_log_external = ConsoleLogExternalFunction::new(
            manager.console_log_js_function.clone(),
            cx.channel(),
            id,
        );
        let emit_external =
            EmitExternalFunction::new(manager.emit_js_function.clone(), cx.channel(), id);
        let inputs_external =
            InputsExternalFunction::new(manager.inputs_js_function.clone(), cx.channel(), id);
        let outputs_external =
            OutputsExternalFunction::new(manager.outputs_js_function.clone(), cx.channel(), id);
        let account_type_external = AccountTypeExternalFunction::new(
            manager.account_type_js_function.clone(),
            cx.channel(),
            id,
        );
        let block_hash_external = BlockHashExternalFunction::new(
            manager.block_hash_js_function.clone(),
            cx.channel(),
            id,
        );

        // Obtain a Runtime from the pool
        let runtime = if let Some(runtime) = manager.runtime_pool.get_runtime() {
            NeonResult::Ok(runtime)
        } else {
            cx.throw_error("No available runtimes in the pool")
        }?;

        if params.memory_pages_used >= MAX_PAGES {
            return cx.throw_error("No more memory pages available");
        }
        let max_pages = MAX_PAGES - params.memory_pages_used;

        //let runtime = Arc::new(Runtime::new()?);
        let custom_env = CustomEnv::new(
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
        )
        .or_else(|err| cx.throw_error(err.to_string()))?;

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
            .or_else(|err| cx.throw_error(err.to_string()))?;
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
                .or_else(|err| cx.throw_error(err.to_string()))?;
            }
        } else {
            return cx.throw_error("No bytecode or serialized data");
        }

        let contract = Contract::from_runner(
            runner,
            params.max_gas,
            runtime.clone(),
            manager.runtime_pool.clone(),
        )
        .or_else(|err| cx.throw_error(err.to_string()))?;

        log_time_diff(&time, "JsContract::from");

        Ok(contract)
    }

    pub fn set_environment_variables(
        &self,
        environment_variables: EnvironmentVariablesRequest,
    ) -> anyhow::Result<()> {
        let mut contract = self
            .contract
            .lock()
            .or_else(|e| Err(anyhow!(e.to_string())))?;
        contract
            .set_environment_variables(environment_variables.into())
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(())
    }

    pub fn on_deploy(&self, calldata: Vec<u8>) -> anyhow::Result<ExitData> {
        // Lock the contract and call
        println!("Ondeploy contract inside");
        let mut contract = self
            .contract
            .lock()
            .or_else(|e| Err(anyhow!(e.to_string())))?;

        println!("Ondeploy contract inside - unlocked");
        let call_result = contract.on_deploy(calldata);

        match call_result {
            Ok(values) => Ok(values),
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }

    pub fn execute(&self, calldata: Vec<u8>) -> anyhow::Result<ExitData> {
        println!("Execute contract inside");
        let time = Local::now();
        // Lock the contract and call
        let mut contract = self
            .contract
            .lock()
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;

        println!("Execute contract inside - unlocked");

        let call_result = contract.execute(calldata);

        println!("Execute contract inside - done");

        let result = match call_result {
            Ok(values) => Ok(values),

            // TODO: LEAVE LIKE THIS FOR NOW, SEE IF WE CAN STOP THE ERROR: ERROR: ERROR EVENTUALLY
            // FIXED THE MASSIVE ERROR OVERFLOW OF ERROR USING ROOT_CAUSE
            Err(e) => Err(anyhow::anyhow!(e.root_cause().to_string())),
        };

        log_time_diff(&time, "JsContract::execute");

        result
    }

    pub async fn call_export_by_name(
        &self,
        function_name: &str,
        int_params: &[i32],
    ) -> anyhow::Result<Box<[Value]>> {
        // Convert the i32s to Wasmer `Value`
        let wasm_params: Vec<Value> = int_params.iter().map(|i| Value::I32(*i)).collect();

        // Lock the contract and call
        let mut contract = self
            .contract
            .lock()
            .or_else(|e| Err(anyhow!(e.to_string())))?;

        let call_result = contract.call_export_by_name(function_name, &wasm_params);

        // Return the raw Values for later JS conversion
        match call_result {
            Ok(values) => Ok(values),
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }

    fn from_runner(
        runner: WasmerRunner,
        max_gas: u64,
        runtime: Arc<Runtime>,
        runtime_pool: Arc<RuntimePool>,
    ) -> anyhow::Result<Self> {
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

    pub fn serialize(&self) -> anyhow::Result<Bytes> {
        let runner = self
            .runner
            .lock()
            .or_else(|e| Err(anyhow!(e.to_string())))?;

        let serialized = runner
            .serialize()
            .map_err(|e| anyhow::anyhow!(format!("{:?}", e)))?;

        Ok(serialized)
    }

    pub fn read_memory(&self, offset: u64, length: u64) -> anyhow::Result<Vec<u8>> {
        let contract = self
            .contract
            .lock()
            .or_else(|e| Err(anyhow!(e.to_string())))?;

        contract.read_memory(offset, length)
    }

    pub fn write_memory(&self, offset: u64, data: &[u8]) -> anyhow::Result<()> {
        let contract = self
            .contract
            .lock()
            .or_else(|e| Err(anyhow!(e.to_string())))?;

        contract
            .write_memory(offset, &data)
            .map_err(|e| anyhow::anyhow!(format!("{:?}", e)))?;

        Ok(())
    }

    pub fn get_used_gas(&self) -> anyhow::Result<u64> {
        let mut contract = self
            .contract
            .lock()
            .or_else(|e| Err(anyhow!(e.to_string())))?;

        contract.get_used_gas()
    }

    pub fn use_gas(&self, gas: u64) -> anyhow::Result<()> {
        let mut contract = self
            .contract
            .lock()
            .or_else(|e| Err(anyhow!(e.to_string())))?;
        contract.use_gas(gas)
    }

    pub fn get_exit_data(&self) -> anyhow::Result<ExitData> {
        let contract = self
            .contract
            .lock()
            .or_else(|e| Err(anyhow!(e.to_string())))?;

        contract.get_exit_data()
    }
}

impl Drop for Contract {
    fn drop(&mut self) {
        // Return the runtime to the pool
        if let Err(e) = self.runtime_pool.return_runtime(self.runtime.clone()) {
            eprintln!("Failed to return runtime to pool: {:?}", e);
        }
    }
}
