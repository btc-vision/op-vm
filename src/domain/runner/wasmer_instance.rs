use std::sync::Arc;

use chrono::Local;
use wasmer::{
    CompilerConfig, ExportError, Function, FunctionEnv, FunctionEnvMut, imports, Imports, Instance,
    Memory, MemoryAccessError, Module, RuntimeError, Store, StoreMut, Value,
};
use wasmer::sys::{BaseTunables, EngineBuilder};
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints, set_remaining_points};
use wasmer_middlewares::Metering;
use wasmer_types::Target;

use crate::domain::contract::AbortData;
use crate::domain::runner::{CustomEnv, RunnerInstance};
use crate::domain::vm::{get_gas_cost, LimitingTunables, log_time_diff};
use crate::interfaces::{CallOtherContractExternalFunction, ConsoleLogExternalFunction, DeployFromAddressExternalFunction, EncodeAddressExternalFunction, ExternalFunction, StorageLoadExternalFunction, StorageStoreExternalFunction};

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

        fn abort(
            mut env: FunctionEnvMut<CustomEnv>,
            message: u32,
            file_name: u32,
            line: u32,
            column: u32,
        ) -> Result<(), RuntimeError> {
            let data = env.data_mut();
            data.abort_data = Some(AbortData {
                message,
                file_name,
                line,
                column,
            });

            return Err(RuntimeError::new("Execution aborted"));
        }

        fn handle_import_call(
            env: &CustomEnv,
            mut store: &mut StoreMut,
            external_function: &impl ExternalFunction,
            ptr: u32,
        ) -> Result<u32, RuntimeError> {
            let memory = env.memory.clone().ok_or(RuntimeError::new("Memory not found"))?;
            let instance = env.instance.clone().ok_or(RuntimeError::new("Instance not found"))?;

            let view = memory.view(&store);

            let data = env
                .read_buffer(&view, ptr)
                .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

            let result = external_function.execute(&data)?;

            let value = env
                .write_buffer(&instance, &mut store, &result, 13, 0)
                .map_err(|_e| RuntimeError::new("Error writing buffer"))?;

            Ok(value as u32)
        }

        fn handle_console_log(
            mut context: FunctionEnvMut<CustomEnv>,
            ptr: u32,
        ) -> Result<(), RuntimeError> {
            let (env, store) = context.data_and_store_mut();
            let memory = env.memory.clone().ok_or(RuntimeError::new("Memory not found"))?;
            let view = memory.view(&store);

            let data = env
                .read_buffer(&view, ptr)
                .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

            env.console_log_external.execute(&data)
        }

        fn sha256_internal(
            mut context: FunctionEnvMut<CustomEnv>,
            ptr: u32,
        ) -> Result<u32, RuntimeError> {
            let (env, mut store) = context.data_and_store_mut();

            let memory = env.memory.clone().ok_or(RuntimeError::new("Memory not found"))?;
            let instance = env.instance.clone().ok_or(RuntimeError::new("Instance not found"))?;

            let view = memory.view(&store);

            let data = env
                .read_buffer(&view, ptr)
                .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

            let result = env.sha256(&data)?;

            let value = env
                .write_buffer(&instance, &mut store, &result, 13, 0)
                .map_err(|_e| RuntimeError::new("Error writing buffer"))?;

            Ok(value as u32)
        }

        macro_rules! import_external {
            ($func:tt, $external:ident) => {{
                fn $func(
                    mut context: FunctionEnvMut<CustomEnv>,
                    ptr: u32,
                ) -> Result<u32, RuntimeError> {
                    let (env, mut store) = context.data_and_store_mut();
                    handle_import_call(env, &mut store, &env.$external, ptr)
                }

                import!($func)
            }};
        }

        macro_rules! import {
            ($func:tt) => {
                Function::new_typed_with_env(&mut store, &env, $func)
            };
        }

        let import_object: Imports = imports! {
            "env" => {
                "abort" => import!(abort),
                "load" => import_external!(storage_load, storage_load_external),
                "store" => import_external!(storage_store, storage_store_external),
                "call" => import_external!(call_other_contract, call_other_contract_external),
                "deployFromAddress" => import_external!(deploy_from_address, deploy_from_address_external),
                "log" => import!(handle_console_log),
                "encodeAddress" => import_external!(encode_address, encode_address_external),
                "sha256" => import!(sha256_internal),
                //"memory" => memory.clone(),
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
