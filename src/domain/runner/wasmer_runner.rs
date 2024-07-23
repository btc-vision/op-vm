use std::sync::Arc;

use chrono::Local;
use wasmer::{
    CompilerConfig, Function, FunctionEnv, imports, Imports, Instance, MemoryAccessError, Module,
    Store, Value,
};
use wasmer::sys::{BaseTunables, EngineBuilder};
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::Metering;
use wasmer_types::Target;

use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::{
    abort_import, AbortData, call_other_contract_import, console_log_import,
    ContractRunner, CustomEnv, deploy_from_address_import, encode_address_import, InstanceWrapper,
    sha256_import, storage_load_import, storage_store_import,
};
use crate::domain::runner::bitcoin_network::BitcoinNetwork;
use crate::domain::vm::{get_gas_cost, LimitingTunables, log_time_diff};
use crate::interfaces::{
    CallOtherContractExternalFunction, ConsoleLogExternalFunction,
    DeployFromAddressExternalFunction, StorageLoadExternalFunction,
    StorageStoreExternalFunction,
};

const MAX_PAGES: u32 = 128 * 4; // 1 page = 64KB, 32 MB.
const STACK_SIZE: usize = 1024 * 1024; // 1MB

pub struct WasmerRunner {
    store: Store,
    instance: InstanceWrapper,
    env: FunctionEnv<CustomEnv>,
}

impl WasmerRunner {
    pub fn new(
        bytecode: &[u8],
        max_gas: u64,
        network: BitcoinNetwork,
        storage_load_external: StorageLoadExternalFunction,
        storage_store_external: StorageStoreExternalFunction,
        call_other_contract_external: CallOtherContractExternalFunction,
        deploy_from_address_external: DeployFromAddressExternalFunction,
        console_log_external: ConsoleLogExternalFunction,
    ) -> anyhow::Result<Self> {
        let time = Local::now();
        let metering = Arc::new(Metering::new(max_gas, get_gas_cost));

        let mut compiler = Singlepass::default();
        compiler.canonicalize_nans(true);
        compiler.push_middleware(metering);
        compiler.enable_verifier();

        let base = BaseTunables::for_target(&Target::default());
        let tunables = LimitingTunables::new(base, MAX_PAGES, STACK_SIZE);

        let mut engine = EngineBuilder::new(compiler).set_features(None).engine();
        engine.set_tunables(tunables);

        let mut store = Store::new(engine);
        let instance = CustomEnv::new(
            network,
            storage_load_external,
            storage_store_external,
            call_other_contract_external,
            deploy_from_address_external,
            console_log_external,
        )?;

        let env = FunctionEnv::new(&mut store, instance);

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
            }
        };

        let module = Module::new(&store, &bytecode)?;
        let instance = Instance::new(&mut store, &module, &import_object)?;
        let instance_wrapper = InstanceWrapper::new(instance.clone());

        env.as_mut(&mut store).instance = Some(instance_wrapper.clone());

        log_time_diff(&time, "WasmerInstance::new");

        Ok(Self {
            store,
            instance: instance_wrapper,
            env,
        })
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
