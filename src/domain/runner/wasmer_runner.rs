use bytes::Bytes;
use chrono::Local;
use std::sync::Arc;
use wasmer::sys::{BaseTunables, EngineBuilder};
use wasmer::{
    imports, CompilerConfig, Function, FunctionEnv, Instance, Module, RuntimeError, Store, Value,
};
use wasmer_compiler::types::target::Target;
use wasmer_compiler::Engine;
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::Metering;
use wasmer_types::SerializeError;

use crate::domain::vm::{get_gas_cost, log_time_diff, LimitingTunables};

use crate::domain::runner::constants::STACK_SIZE;
#[allow(unused_imports)]
use crate::domain::runner::{
    CallOtherContractImport, Calldata, ConsoleLogImport, ContractRunner, CustomEnv,
    DeployFromAddressImport, EmitImport, EnvironmentVariables, ExitData, ExitImport, ExitResult,
    ExtendedMemoryAccessError, GetAccountTypeImport, GetBlockHashImport, GetCallResultImport,
    GetCalldataImport, GetEnvironmentVariablesImport, GetInputsImport, GetInputsSizeImport,
    GetOutputsImport, GetOutputsSizeImport, InstanceWrapper, Ripemd160Import, Sha256Import,
    StorageLoadImport, StorageStoreImport, TransientLoadImport, TransientStoreImport,
    ValidateBitcoinAddressImport, VerifySchnorrImport, MAX_GAS_WASM_INIT,
};

const CONTRACT_ENTRYPOINT_FUNCTION_NAME: &'static str = "execute";
const CONTRACT_ON_DEPLOY_FUNCTION_NAME: &'static str = "onDeploy";

pub struct WasmerRunner {
    module: Module,
    store: Store,
    instance: InstanceWrapper,
    env: FunctionEnv<CustomEnv>,
}

impl WasmerRunner {
    pub fn from_bytecode(
        bytecode: &[u8],
        used_gas: u64,
        max_gas: u64,
        max_pages: u32,
        custom_env: CustomEnv,
        is_debug_mode: bool,
    ) -> anyhow::Result<Self> {
        let time = Local::now();

        let store = Self::create_engine(max_pages)?;
        let module = Module::from_binary(&store, &bytecode)?;
        let instance =
            Self::create_instance(used_gas, max_gas, custom_env, store, module, is_debug_mode)?;

        log_time_diff(&time, "WasmerRunner::from_bytecode");

        Ok(instance)
    }

    fn create_engine(max_pages: u32) -> anyhow::Result<Store> {
        let meter = Metering::new(MAX_GAS_WASM_INIT, get_gas_cost);
        let metering = Arc::new(meter);

        let mut compiler = Singlepass::default();
        compiler.canonicalize_nans(true);
        compiler.push_middleware(metering);
        compiler.enable_verifier();

        let engine = EngineBuilder::new(compiler).set_features(None).engine();
        let store = Store::new(Self::create_tunable(engine, max_pages));
        Ok(store)
    }

    pub fn serialize(&self) -> anyhow::Result<Bytes, SerializeError> {
        let serialized = self.module.serialize()?;

        Ok(serialized)
    }

    pub unsafe fn from_serialized(
        serialized: Bytes,
        used_gas: u64,
        max_gas: u64,
        max_pages: u32,
        custom_env: CustomEnv,
        is_debug_mode: bool,
    ) -> anyhow::Result<Self> {
        let time = Local::now();

        let engine = EngineBuilder::headless().set_features(None).engine();
        let store = Store::new(Self::create_tunable(engine, max_pages));
        let module = Module::deserialize(&store, serialized)?;
        let instance =
            Self::create_instance(used_gas, max_gas, custom_env, store, module, is_debug_mode)?;

        log_time_diff(&time, "WasmerRunner::from_serialized");

        Ok(instance)
    }

    fn create_tunable(mut engine: Engine, max_pages: u32) -> Engine {
        let base = BaseTunables::for_target(&Target::default());
        let tunables = LimitingTunables::new(base, max_pages, STACK_SIZE);

        engine.set_tunables(tunables);

        engine
    }

    fn create_instance(
        used_gas: u64,
        max_gas: u64,
        custom_env: CustomEnv,
        mut store: Store,
        module: Module,
        is_debug_mode: bool,
    ) -> anyhow::Result<Self> {
        // Verify and calculate remaining gas
        let remaining_gas = Self::calculate_remaining_gas(used_gas, max_gas)?;

        // Load environment
        let env = FunctionEnv::new(&mut store, custom_env);

        macro_rules! import {
            ($obj:tt) => {
                Function::new_typed_with_env(&mut store, &env, $obj::execute)
            };
        }

        let mut import_object = imports! {
            "env" => {
                "exit" => import!(ExitImport),
                "environment" => import!(GetEnvironmentVariablesImport),
                "calldata" => import!(GetCalldataImport),
                "load" => import!(StorageLoadImport),
                "store" => import!(StorageStoreImport),
                // "tload" => import!(TransientLoadImport),
                // "tstore" => import!(TransientStoreImport),
                "call" => import!(CallOtherContractImport),
                "callResult" => import!(GetCallResultImport),
                "deployFromAddress" => import!(DeployFromAddressImport),
                "emit" => import!(EmitImport),
                "inputs" => import!(GetInputsImport),
                "inputsSize" => import!(GetInputsSizeImport),
                "outputs" => import!(GetOutputsImport),
                "outputsSize" => import!(GetOutputsSizeImport),
                "sha256" => import!(Sha256Import),
                "ripemd160" => import!(Ripemd160Import),
                // "accountType" => import!(GetAccountTypeImport),
                // "blockHash" => import!(GetBlockHashImport),
                "validateBitcoinAddress" => import!(ValidateBitcoinAddressImport),
                "verifySchnorrSignature" => import!(VerifySchnorrImport),
            },
        };

        if is_debug_mode {
            import_object.define("debug", "log", import!(ConsoleLogImport));
        }

        let instance_result = Instance::new(&mut store, &module, &import_object);
        let instance = match instance_result {
            Ok(i) => i,
            Err(e) => {
                if e.to_string().contains("unreachable") {
                    return Err(anyhow::anyhow!(
                        "out of gas during initialization (consumed: {})",
                        MAX_GAS_WASM_INIT
                    ));
                }

                return Err(anyhow::anyhow!("Failed to instantiate contract: {}", e));
            }
        };

        let instance_wrapper = InstanceWrapper::new(instance.clone(), max_gas);
        env.as_mut(&mut store).instance = Some(instance_wrapper.clone());
        env.as_mut(&mut store).instance = Some(instance_wrapper.clone());

        let mut imp = Self {
            module,
            store,
            instance: instance_wrapper,
            env,
        };

        // We must verify that the gas we do not track yet is not greater than the remaining gas
        let gas_used_by_start_function = imp.get_remaining_gas();
        if gas_used_by_start_function > remaining_gas {
            return Err(anyhow::anyhow!("out of gas (consumed: {})", max_gas));
        }

        // Get the remaining gas after initializing the contract
        let remaining_gas = remaining_gas - gas_used_by_start_function;

        // Now, we override the default remaining gas with the actual remaining gas
        imp.set_remaining_gas(remaining_gas);

        // Start explicitly the contract "start" function
        let start_function = instance
            .exports
            .get_function("start")
            .map_err(|_| anyhow::anyhow!("OP_NET: start function not found"))?;

        // Call the "start" function and handle the result
        imp.set_is_running_start(true);
        let result_start = start_function.call(&mut imp.store, &[]);
        imp.set_is_running_start(false);

        // Handle the result of the "start" function
        if let Err(e) = result_start {
            if e.to_string().contains("unreachable") {
                return Err(anyhow::anyhow!(
                    "out of gas during start function (consumed: {})",
                    max_gas
                ));
            }

            let exit_data_clone = imp.env.as_ref(&imp.store).exit_data.clone();
            if !exit_data_clone.is_ok() {
                return Err(anyhow::anyhow!(
                    "Failed to call start function: {}",
                    exit_data_clone
                ));
            }

            return Err(anyhow::anyhow!("Failed to call start function: {}", e));
        }

        Ok(imp)
    }

    fn calculate_remaining_gas(used_gas: u64, max_gas: u64) -> anyhow::Result<u64> {
        if MAX_GAS_WASM_INIT > max_gas {
            return Err(anyhow::anyhow!(
                "too little gas, minimum is {}",
                MAX_GAS_WASM_INIT
            ));
        }

        if used_gas > max_gas {
            return Err(anyhow::anyhow!(
                "too little gas remaining (used: {}, max: {})",
                used_gas,
                max_gas
            ));
        }

        let remaining_gas = max_gas - used_gas;
        if remaining_gas < MAX_GAS_WASM_INIT {
            return Err(anyhow::anyhow!(
                "too little gas remaining, minimum is {}",
                MAX_GAS_WASM_INIT
            ));
        }

        // Always charge the maximum gas for the initialization, we have no way of calculating the
        // actual gas used since this is done in internal wasmer functions and this is used to inject
        // metering. This is a safe operation since we have already checked that the remaining gas
        // is enough.
        Ok(remaining_gas)
    }

    fn set_is_running_start(&mut self, value: bool) {
        let env = self.env.as_mut(&mut self.store);
        env.is_running_start_function = value;
    }

    fn handle_errors(
        &mut self,
        response: Result<Box<[Value]>, RuntimeError>,
        max_gas: u64,
    ) -> anyhow::Result<Box<[Value]>> {
        response.map_err(|e| {
            if e.to_string().contains("unreachable") {
                let gas_used = self.get_remaining_gas();
                if gas_used == 0 {
                    anyhow::anyhow!("out of gas (consumed: {})", max_gas)
                } else {
                    let out_of_memory = self.is_out_of_memory().unwrap_or(false);

                    if out_of_memory {
                        anyhow::anyhow!("out of memory")
                    } else {
                        anyhow::anyhow!(e)
                    }
                }
            } else {
                anyhow::anyhow!(e)
            }
        })
    }
}

impl ContractRunner for WasmerRunner {
    fn set_environment_variables(&mut self, environment_variables: EnvironmentVariables) {
        let env = self.env.as_mut(&mut self.store);
        env.environment_variables = Some(environment_variables);
    }

    fn on_deploy(&mut self, calldata: &[u8], max_gas: u64) -> anyhow::Result<ExitData> {
        let env = self.env.as_mut(&mut self.store);
        env.calldata = Calldata::new(&calldata);

        let export = self
            .instance
            .get_function(CONTRACT_ON_DEPLOY_FUNCTION_NAME)?;

        let params = &[Value::I32(calldata.len() as i32)];
        let response = export.call(&mut self.store, params);

        let response: Result<Box<[Value]>, RuntimeError> = match response {
            Ok(result) => Ok(result),
            Err(error) => match error.downcast::<ExitResult>() {
                Ok(result) => match result {
                    ExitResult::Ok(data) => return Ok(data),
                    ExitResult::Err(e) => Err(e)?,
                },
                Err(e) => Err(e),
            },
        };

        let result = self.handle_errors(response, max_gas)?;

        let status = result
            .get(0)
            .ok_or(RuntimeError::new("Invalid value returned from contract"))?
            .i32()
            .ok_or(RuntimeError::new("Invalid value returned from contract"))?
            as u32;

        let gas_used = self.get_used_gas();
        let env = self.env.as_mut(&mut self.store);

        env.exit_data = ExitData::new(status, gas_used, &[]);

        Ok(env.exit_data.clone())
    }

    fn execute(&mut self, calldata: &[u8], max_gas: u64) -> anyhow::Result<ExitData> {
        let time = Local::now();
        let env = self.env.as_mut(&mut self.store);
        env.calldata = Calldata::new(&calldata);

        let export = self
            .instance
            .get_function(CONTRACT_ENTRYPOINT_FUNCTION_NAME)?;

        let params = &[Value::I32(calldata.len() as i32)];
        let response = export.call(&mut self.store, params);

        let response: Result<Box<[Value]>, RuntimeError> = match response {
            Ok(result) => Ok(result),
            Err(error) => match error.downcast::<ExitResult>() {
                Ok(result) => match result {
                    ExitResult::Ok(data) => return Ok(data),
                    ExitResult::Err(e) => Err(e)?,
                },
                Err(e) => Err(e),
            },
        };

        let result = self.handle_errors(response, max_gas)?;

        let status = result
            .get(0)
            .ok_or(RuntimeError::new("Invalid value returned from contract"))?
            .i32()
            .ok_or(RuntimeError::new("Invalid value returned from contract"))?
            as u32;

        let gas_used = self.get_used_gas();
        let env = self.env.as_mut(&mut self.store);
        env.exit_data = ExitData::new(status, gas_used, &[]);

        log_time_diff(&time, "WasmerRunner::execute");
        Ok(env.exit_data.clone())
    }

    fn call_export_by_name(
        &mut self,
        function_name: &str,
        params: &[Value],
        max_gas: u64,
    ) -> anyhow::Result<Box<[Value]>> {
        let export = self.instance.get_function(function_name)?;
        let response = export.call(&mut self.store, params);
        self.handle_errors(response, max_gas)
    }

    fn read_memory(&self, offset: u64, length: u64) -> Result<Vec<u8>, ExtendedMemoryAccessError> {
        self.instance.read_memory(&self.store, offset, length)
    }

    fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), ExtendedMemoryAccessError> {
        self.instance.write_memory(&self.store, offset, data)
    }

    fn get_remaining_gas(&mut self) -> u64 {
        self.instance.get_remaining_gas(&mut self.store)
    }

    fn get_used_gas(&mut self) -> u64 {
        self.instance.get_used_gas(&mut self.store)
    }

    fn is_out_of_memory(&self) -> Result<bool, ExtendedMemoryAccessError> {
        self.instance.is_out_of_memory(&self.store)
    }

    fn set_remaining_gas(&mut self, gas: u64) {
        self.instance.set_remaining_gas(&mut self.store, gas)
    }

    fn use_gas(&mut self, gas: u64) {
        self.instance.use_gas(&mut self.store, gas)
    }

    fn get_exit_data(&self) -> ExitData {
        self.env.as_ref(&self.store).exit_data.clone()
    }
}
