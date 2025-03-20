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

use crate::domain::runner::constants::{MAX_PAGES, STACK_SIZE};
use crate::domain::runner::{
    AddressTypeImport, BlockHashImport, CallOtherContractImport, Calldata, ConsoleLogImport,
    ContractRunner, CustomEnv, DeployFromAddressImport, EmitImport, EnvironmentVariables, ExitData,
    ExitImport, ExitResult, ExtendedMemoryAccessError, GetCallResultImport, GetCalldataImport,
    GetEnvironmentVariablesImport, GetInputsImport, GetInputsSizeImport, GetOutputsImport,
    GetOutputsSizeImport, InstanceWrapper, Ripemd160Import, Sha256Import, StorageLoadImport,
    StorageStoreImport, TransientStorageLoadImport, TransientStorageStoreImport,
    ValidateBitcoinAddressImport, VerifySchnorrImport,
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
    pub fn validate_bytecode(bytecode: &[u8], max_gas: u64) -> anyhow::Result<bool> {
        let time = Local::now();
        let metering = Arc::new(Metering::new(max_gas, get_gas_cost));

        let mut compiler = Singlepass::default();
        compiler.canonicalize_nans(true);
        compiler.push_middleware(metering);
        compiler.enable_verifier();

        let engine = EngineBuilder::new(compiler).set_features(None).engine();
        let store = Store::new(engine);

        Module::validate(&store, &bytecode)?;

        log_time_diff(&time, "WasmerRunner::validate_bytecode");

        Ok(true)
    }

    pub fn from_bytecode(
        bytecode: &[u8],
        max_gas: u64,
        custom_env: CustomEnv,
        is_debug_mode: bool,
    ) -> anyhow::Result<Self> {
        let time = Local::now();

        let store = Self::create_engine(max_gas)?;
        let module = Module::from_binary(&store, &bytecode)?;
        let instance = Self::create_instance(max_gas, custom_env, store, module, is_debug_mode)?;

        log_time_diff(&time, "WasmerRunner::from_bytecode");

        Ok(instance)
    }

    pub fn serialize(&self) -> anyhow::Result<Bytes, SerializeError> {
        let serialized = self.module.serialize()?;

        Ok(serialized)
    }

    pub unsafe fn from_serialized(
        serialized: Bytes,
        max_gas: u64,
        custom_env: CustomEnv,
        is_debug_mode: bool,
    ) -> anyhow::Result<Self> {
        let time = Local::now();

        let engine = EngineBuilder::headless().set_features(None).engine();
        let store = Store::new(Self::create_tunable(engine));
        let module = Module::deserialize(&store, serialized)?;
        let instance = Self::create_instance(max_gas, custom_env, store, module, is_debug_mode)?;

        log_time_diff(&time, "WasmerRunner::from_serialized");

        Ok(instance)
    }

    fn create_instance(
        max_gas: u64,
        custom_env: CustomEnv,
        mut store: Store,
        module: Module,
        is_debug_mode: bool,
    ) -> anyhow::Result<Self> {
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
                "tload" => import!(TransientStorageLoadImport),
                "tstore" => import!(TransientStorageStoreImport),
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
                "address_type" => import!(AddressTypeImport),
                "block_hash" => import!(BlockHashImport),
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
                        "constructor reached an unreachable opcode (out of gas?)"
                    ));
                }

                return Err(anyhow::anyhow!("Failed to instantiate contract: {}", e));
            }
        };

        let instance_wrapper = InstanceWrapper::new(instance.clone(), max_gas);
        env.as_mut(&mut store).instance = Some(instance_wrapper.clone());

        let mut imp = Self {
            module,
            store,
            instance: instance_wrapper,
            env,
        };

        imp.set_remaining_gas(max_gas);

        // Start explicitly
        let start_function = instance
            .exports
            .get_function("start")
            .map_err(|_| anyhow::anyhow!("OP_NET: start function not found"))?;

        let env = imp.env.as_mut(&mut imp.store);
        env.is_running_start_function = true;
        let result_start = start_function.call(&mut imp.store, &[]);
        let env = imp.env.as_mut(&mut imp.store);
        env.is_running_start_function = false;

        if let Err(e) = result_start {
            if e.to_string().contains("unreachable") {
                return Err(anyhow::anyhow!(
                    "start function reached an unreachable opcode (out of gas?)"
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

    fn create_tunable(mut engine: Engine) -> Engine {
        let base = BaseTunables::for_target(&Target::default());
        let tunables = LimitingTunables::new(base, MAX_PAGES, STACK_SIZE);

        engine.set_tunables(tunables);

        engine
    }

    fn create_engine(max_gas: u64) -> anyhow::Result<Store> {
        let meter = Metering::new(max_gas, get_gas_cost);
        let metering = Arc::new(meter);

        let mut compiler = Singlepass::default();
        compiler.canonicalize_nans(true);
        compiler.push_middleware(metering);
        compiler.enable_verifier();

        let engine = EngineBuilder::new(compiler).set_features(None).engine();
        let store = Store::new(Self::create_tunable(engine));
        Ok(store)
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

        let env = self.env.as_mut(&mut self.store);
        env.exit_data = ExitData::new(status, &[]);

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

        let env = self.env.as_mut(&mut self.store);
        env.exit_data = ExitData::new(status, &[]);

        let result = env.exit_data.clone();

        log_time_diff(&time, "WasmerRunner::execute");
        Ok(result)
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
