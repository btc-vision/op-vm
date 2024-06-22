use std::sync::Arc;

use napi::bindgen_prelude::BigInt;
use napi::Status;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use wasmer::{
    CompilerConfig, ExportError, Function, FunctionEnv, FunctionEnvMut, imports, Instance, Memory,
    MemoryAccessError, Module, RuntimeError, Store, Value,
};
use wasmer::sys::{BaseTunables, EngineBuilder};
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints, set_remaining_points};
use wasmer_middlewares::Metering;
use wasmer_types::{Pages, Target};

use crate::domain::contract::{AbortData, CustomEnv};
use crate::domain::runner::RunnerInstance;
use crate::domain::vm::{get_op_cost, LimitingTunables};

pub struct WasmerInstance {
    store: Store,
    instance: Instance,
    env: FunctionEnv<CustomEnv>,
}

impl WasmerInstance {
    pub fn new(bytecode: &[u8], max_gas: u64, load_function: Arc<ThreadsafeFunction<BigInt, ErrorStrategy::Fatal>>, store_function: Arc<ThreadsafeFunction<BigInt, ErrorStrategy::Fatal>>) -> anyhow::Result<Self> {
        let metering = Arc::new(Metering::new(max_gas, get_op_cost));

        let mut compiler = Singlepass::default();
        compiler.canonicalize_nans(true);
        compiler.push_middleware(metering);
        compiler.enable_verifier();

        let base = BaseTunables::for_target(&Target::default());
        let tunables = LimitingTunables::new(base, Pages(16)); // 1 page = 64 KiB

        let mut engine = EngineBuilder::new(compiler).set_features(None).engine();
        engine.set_tunables(tunables);

        let mut store = Store::new(engine);

        let env = FunctionEnv::new(&mut store, CustomEnv {
            abort_data: None,
            load_function,
            store_function,
        });

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

        fn load(mut env: &FunctionEnvMut<CustomEnv>, pointer: u32) -> Result<(), RuntimeError> {
            let data = env.data_mut();
            let js_pointer: BigInt = BigInt::from(pointer);

            let result: Status = data.load_function.call(js_pointer, napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);

            match result {
                Ok(value) => {
                    let js_value: JsUnknown = value.await.unwrap();
                    let js_bigint: BigInt = js_value.try_into()?;
                    Ok(js_bigint)
                }
                Err(e) => Err(Error::new(Status::GenericFailure, format!("Failed to call JavaScript function: {:?}", e))),
            }
        }

        fn store_fn(mut env: &FunctionEnvMut<CustomEnv>, offset: u32, data: &[u8]) -> Result<(), RuntimeError> {
            let result = data.store(offset, data);
            result
        }

        let abort_typed = Function::new_typed_with_env(&mut store, &env, abort);
        let load_typed = Function::new_typed_with_env(&mut store, &env, load);
        let store_typed = Function::new_typed_with_env(&mut store, &env, store_fn);

        let import_object = imports! {
            "env" => {
                "abort" => abort_typed,
                "load" => load_typed,
                "store" => store_typed
            }
        };

        let module: Module = Module::new(&store, &bytecode)?;
        let instance: Instance = Instance::new(&mut store, &module, &import_object)?;

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
        view.read(offset, &mut buffer).unwrap();

        Ok(buffer)
    }

    fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), MemoryAccessError> {
        let memory = Self::get_memory(&self.instance);
        let view = memory.view(&self.store);
        return view.write(offset, data);
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
