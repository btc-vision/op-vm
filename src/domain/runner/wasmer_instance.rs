use std::sync::Arc;

use chrono::Local;
use wasmer::{
    CompilerConfig, ExportError, Function, FunctionEnv, imports, Imports, Instance,
    Memory, MemoryAccessError, Module, RuntimeError, Store, Value,
};
use wasmer::sys::{BaseTunables, EngineBuilder};
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints, set_remaining_points};
use wasmer_middlewares::Metering;
use wasmer_types::Target;

use crate::domain::contract::AbortData;
use crate::domain::runner::{abort_import, call_other_contract_import, console_log_import, CustomEnv, deploy_from_address_import, encode_address_import, RunnerInstance, sha256_import, storage_load_import, storage_store_import};
use crate::domain::vm::{get_gas_cost, LimitingTunables, log_time_diff};
use crate::interfaces::{CallOtherContractExternalFunction, ConsoleLogExternalFunction, DeployFromAddressExternalFunction, EncodeAddressExternalFunction, StorageLoadExternalFunction, StorageStoreExternalFunction};

const MAX_PAGES: u32 = 128; // 1 page = 64KB
const STACK_SIZE: usize = 1024 * 1024; // 1MB

pub struct WasmerInstance {
    store: Store,
    instance: Instance,
    env: FunctionEnv<CustomEnv>,
}

impl WasmerInstance {
    pub fn new(
        bytecode: &[u8],
        max_gas: u64,
        storage_load_external: StorageLoadExternalFunction,
        storage_store_external: StorageStoreExternalFunction,
        call_other_contract_external: CallOtherContractExternalFunction,
        deploy_from_address_external: DeployFromAddressExternalFunction,
        console_log_external: ConsoleLogExternalFunction,
        encode_address_external: EncodeAddressExternalFunction,
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
            storage_load_external,
            storage_store_external,
            call_other_contract_external,
            deploy_from_address_external,
            console_log_external,
            encode_address_external,
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

        env.as_mut(&mut store).memory = Some(Self::get_memory(&instance).clone());
        env.as_mut(&mut store).instance = Some(instance.clone());

        log_time_diff(&time, "WasmerInstance::new");

        Ok(Self {
            store,
            instance,
            env,
        })
    }

    fn get_memory(instance: &Instance) -> &Memory {
        instance.exports.get_memory("memory").unwrap()
    }

    fn get_function<'a>(
        instance: &'a Instance,
        function: &str,
    ) -> Result<&'a Function, ExportError> {
        instance.exports.get_function(function)
    }
}

impl RunnerInstance for WasmerInstance {
    fn call(&mut self, function: &str, params: &[Value]) -> anyhow::Result<Box<[Value]>> {
        let export = Self::get_function(&self.instance, function)?;
        let result = export.call(&mut self.store, params)?;

        Ok(result)
    }

    fn read_memory(&self, offset: u64, length: u64) -> Result<Vec<u8>, RuntimeError> {
        let memory = Self::get_memory(&self.instance);
        let view = memory.view(&self.store);

        let mut buffer: Vec<u8> = vec![0; length as usize];
        view.read(offset, &mut buffer)?;

        Ok(buffer)
    }

    fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), MemoryAccessError> {
        let memory = Self::get_memory(&self.instance);
        let view = memory.view(&self.store);
        view.write(offset, data)
    }

    fn get_remaining_gas(&mut self) -> u64 {
        let remaining_points = get_remaining_points(&mut self.store, &self.instance);
        match remaining_points {
            MeteringPoints::Remaining(remaining) => remaining,
            MeteringPoints::Exhausted => 0,
        }
    }

    fn set_remaining_gas(&mut self, gas: u64) {
        set_remaining_points(&mut self.store, &self.instance, gas);
    }

    fn get_abort_data(&self) -> Option<AbortData> {
        self.env.as_ref(&self.store).abort_data
    }
}
