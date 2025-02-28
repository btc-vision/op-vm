use bytes::Bytes;
use chrono::Local;
use std::sync::Arc;
use wasmer::sys::{BaseTunables, EngineBuilder};
use wasmer::{imports, CompilerConfig, Function, FunctionEnv, Instance, Module, Store, Value};
use wasmer_compiler::types::target::Target;
use wasmer_compiler::Engine;
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::Metering;
use wasmer_types::SerializeError;

use crate::domain::assembly_script::AssemblyScript;
use crate::domain::vm::{get_gas_cost, log_time_diff, LimitingTunables};

use crate::domain::runner::constants::{MAX_GAS_CONSTRUCTOR, MAX_PAGES, STACK_SIZE};
use crate::domain::runner::{CallOtherContractImport, Calldata, ConsoleLogImport, ContractRunner, CustomEnv, DeployFromAddressImport, EmitImport, ExtendedMemoryAccessError, GetCallResultImport, GetCalldataImport, GetInputsSizeImport, GetOuputsSizeImport, InputsImport, InstanceWrapper, OutputsImport, RevertData, RevertImport, Ripemd160Import, Sha256Import, StorageLoadImport, StorageStoreImport, ValidateBitcoinAddressImport, VerifySchnorrImport};

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

        let store = Self::create_engine()?;
        let module = Module::from_binary(&store, &bytecode)?;
        let instance = Self::create_instance(max_gas, custom_env, store, module, is_debug_mode)?;

        log_time_diff(&time, "WasmerInstance::from_bytecode");

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

        log_time_diff(&time, "WasmerInstance::from_serialized");

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
                "revert" => import!(RevertImport),
                "calldata" => import!(GetCalldataImport),
                "load" => import!(StorageLoadImport),
                "store" => import!(StorageStoreImport),
                "call" => import!(CallOtherContractImport),
                "callResult" => import!(GetCallResultImport),
                "deployFromAddress" => import!(DeployFromAddressImport),
                "emit" => import!(EmitImport),
                "inputs" => import!(InputsImport),
                "inputsSize" => import!(GetInputsSizeImport),
                "outputs" => import!(OutputsImport),
                "outputsSize" => import!(GetOuputsSizeImport),
                "sha256" => import!(Sha256Import),
                "ripemd160" => import!(Ripemd160Import),
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

        let instance_wrapper = InstanceWrapper::new(instance.clone());
        env.as_mut(&mut store).instance = Some(instance_wrapper.clone());

        let mut imp = Self {
            module,
            store,
            instance: instance_wrapper,
            env,
        };

        // Start explicitly
        let start_function = instance
            .exports
            .get_function("start")
            .map_err(|_| anyhow::anyhow!("OP_NET: start function not found"))?;
        let result_start = start_function.call(&mut imp.store, &[]);

        if let Err(e) = result_start {
            if e.to_string().contains("unreachable") {
                return Err(anyhow::anyhow!(
                    "start function reached an unreachable opcode (out of gas?)"
                ));
            }

            let revert_data_clone = imp.env.as_ref(&imp.store).revert_data.clone();
            if revert_data_clone.is_some() {
                return Err(anyhow::anyhow!(
                    "Failed to call start function: {}",
                    revert_data_clone.unwrap()
                ));
            }

            return Err(anyhow::anyhow!("Failed to call start function: {}", e));
        }

        let remaining_gas = imp.get_remaining_gas();
        let constructor_used_gas = MAX_GAS_CONSTRUCTOR - remaining_gas;

        let true_max_gas = max_gas - constructor_used_gas;
        imp.set_remaining_gas(true_max_gas);

        Ok(imp)
    }

    fn create_tunable(mut engine: Engine) -> Engine {
        let base = BaseTunables::for_target(&Target::default());
        let tunables = LimitingTunables::new(base, MAX_PAGES, STACK_SIZE);

        engine.set_tunables(tunables);

        engine
    }

    fn create_engine() -> anyhow::Result<Store> {
        let meter = Metering::new(MAX_GAS_CONSTRUCTOR, get_gas_cost);
        let metering = Arc::new(meter);

        let mut compiler = Singlepass::default();
        compiler.canonicalize_nans(true);
        compiler.push_middleware(metering);
        compiler.enable_verifier();

        let engine = EngineBuilder::new(compiler).set_features(None).engine();
        let store = Store::new(Self::create_tunable(engine));
        Ok(store)
    }
}

impl ContractRunner for WasmerRunner {
    fn execute(&mut self, calldata: &[u8]) -> anyhow::Result<Box<[Value]>> {
        let env = self.env.as_mut(&mut self.store);
        env.calldata = Calldata::new(&calldata);

        self.instance.call_entrypoint(&mut self.store, calldata.len() as u32)
    }

    fn call(&mut self, function: &str, params: &[Value]) -> anyhow::Result<Box<[Value]>> {
        self.instance.call(&mut self.store, function, params)
    }

    fn read_memory(&self, offset: u64, length: u64) -> Result<Vec<u8>, ExtendedMemoryAccessError> {
        self.instance.read_memory(&self.store, offset, length)
    }

    fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), ExtendedMemoryAccessError> {
        self.instance.write_memory(&self.store, offset, data)
    }

    fn write_buffer(&mut self, value: &[u8], id: i32, align: u32) -> Result<i64, napi::Error> {
        AssemblyScript::write_buffer(&mut self.store, &self.instance, value, id, align)
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

    fn get_revert_data(&self) -> Option<RevertData> {
        self.env.as_ref(&self.store).revert_data.clone()
    }
}
