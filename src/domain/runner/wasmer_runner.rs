use bytes::Bytes;
use chrono::Local;
use std::sync::Arc;
use wasmer::sys::{BaseTunables, EngineBuilder};
use wasmer::{
    imports, CompilerConfig, Function, FunctionEnv, Imports, Instance, MemoryAccessError, Module,
    Store, Value,
};
use wasmer_compiler::Engine;
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::Metering;
use wasmer_types::{SerializeError, Target};

use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::{
    abort_import, call_other_contract_import, console_log_import, deploy_from_address_import,
    encode_address_import, flush_events_import, sha256_import, storage_load_import,
    storage_store_import, AbortData, ContractRunner, CustomEnv, InstanceWrapper,
};
use crate::domain::vm::{get_gas_cost, log_time_diff, LimitingTunables};

use crate::domain::runner::constants::{MAX_GAS_CONSTRUCTOR, MAX_PAGES, STACK_SIZE};

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
    ) -> anyhow::Result<Self> {
        let time = Local::now();

        let store = Self::create_engine()?;
        let module = Module::from_binary(&store, &bytecode)?;
        let instance = Self::create_instance(max_gas, custom_env, store, module)?;

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
    ) -> anyhow::Result<Self> {
        let time = Local::now();

        let engine = EngineBuilder::headless().set_features(None).engine();
        let store = Store::new(Self::create_tunable(engine));
        let module = Module::deserialize(&store, serialized)?;
        let instance = Self::create_instance(max_gas, custom_env, store, module)?;

        log_time_diff(&time, "WasmerInstance::from_serialized");

        Ok(instance)
    }

    fn create_instance(
        max_gas: u64,
        custom_env: CustomEnv,
        mut store: Store,
        module: Module,
    ) -> anyhow::Result<Self> {
        let env = FunctionEnv::new(&mut store, custom_env);

        macro_rules! import {
            ($func:tt) => {
                Function::new_typed_with_env(&mut store, &env, $func)
            };
        }

        let import_object: Imports = imports! {
            "env" => {
                "abort" => import!(abort_import),
                "load" => import!(storage_load_import),
                "store" => import!(storage_store_import),
                "call" => import!(call_other_contract_import),
                "deployFromAddress" => import!(deploy_from_address_import),
                "encodeAddress" => import!(encode_address_import),
                "sha256" => import!(sha256_import),
                "log" => import!(console_log_import),
                "flushEvents" => import!(flush_events_import)
            }
        };

        let instance = Instance::new(&mut store, &module, &import_object).map_err(|e| {
            if e.to_string().contains("unreachable") {
                anyhow::anyhow!("constructor out of gas")
            } else {
                anyhow::anyhow!(e)
            }
        })?;

        let instance_wrapper = InstanceWrapper::new(instance.clone());
        env.as_mut(&mut store).instance = Some(instance_wrapper.clone());

        let mut imp = Self {
            module,
            store,
            instance: instance_wrapper,
            env,
        };

        let remaining_gas = imp.get_remaining_gas();
        let constructor_used_gas = MAX_GAS_CONSTRUCTOR - remaining_gas;

        let true_max_gas = max_gas - constructor_used_gas;
        imp.set_remaining_gas(true_max_gas);

        Ok(imp)
    }

    /*fn reset(&mut self) {
        let engine = EngineBuilder::headless().set_features(None).engine();
        self.store = Store::new(Self::create_tunable(engine));

        self.set_remaining_gas(MAX_GAS_CONSTRUCTOR);

        let instance = Instance::new(&mut self.store, &self.module, &imports! {}).unwrap();
    }*/

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
    fn call(&mut self, function: &str, params: &[Value]) -> anyhow::Result<Box<[Value]>> {
        self.instance.call(&mut self.store, function, params)
    }

    fn read_memory(&self, offset: u64, length: u64) -> Result<Vec<u8>, MemoryAccessError> {
        self.instance.read_memory(&self.store, offset, length)
    }

    fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), MemoryAccessError> {
        self.instance.write_memory(&self.store, offset, data)
    }

    fn write_buffer(&mut self, value: &[u8], id: i32, align: u32) -> Result<i64, napi::Error> {
        AssemblyScript::write_buffer(&mut self.store, &self.instance, value, id, align)
    }

    fn get_remaining_gas(&mut self) -> u64 {
        self.instance.get_remaining_gas(&mut self.store)
    }

    fn is_out_of_memory(&self) -> Result<bool, MemoryAccessError> {
        self.instance.is_out_of_memory(&self.store)
    }

    fn set_remaining_gas(&mut self, gas: u64) {
        self.instance.set_remaining_gas(&mut self.store, gas)
    }

    fn use_gas(&mut self, gas: u64) {
        self.instance.use_gas(&mut self.store, gas)
    }

    fn get_abort_data(&self) -> Option<AbortData> {
        self.env.as_ref(&self.store).abort_data
    }
}
