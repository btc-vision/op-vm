use std::sync::Arc;

use wasmer::{CompilerConfig, ExportError, Function, FunctionEnv, FunctionEnvMut, imports, Instance, Memory, MemoryAccessError, Module, RuntimeError, Store, Value};
use wasmer::sys::EngineBuilder;
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints};
use wasmer_middlewares::Metering;

use crate::domain::contract::{AbortData, CustomEnv};
use crate::domain::runner::RunnerInstance;
use crate::domain::vm::{get_op_cost, MAX_GAS};

pub struct WasmerInstance {
    store: Store,
    instance: Instance,
    env: FunctionEnv<CustomEnv>,
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

    fn get_abort_data(&self) -> Option<AbortData> {
        self.env.as_ref(&self.store).abort_data
    }
}

impl WasmerInstance {
    pub fn new(bytecode: &[u8]) -> Self {
        let metering = Arc::new(Metering::new(MAX_GAS, get_op_cost));

        let mut compiler = Singlepass::default();
        compiler.canonicalize_nans(true);
        compiler.push_middleware(metering);

        let engine = EngineBuilder::new(compiler).set_features(None).engine();
        let mut store = Store::new(engine);

        let env = FunctionEnv::new(&mut store, CustomEnv { abort_data: None });

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

        let abort_typed = Function::new_typed_with_env(&mut store, &env, abort);
        let import_object = imports! {
            "env" => {
                "abort" => abort_typed,
            }
        };

        let module = Module::new(&store, &bytecode).unwrap();
        let instance = Instance::new(&mut store, &module, &import_object).unwrap();

        Self {
            store,
            instance,
            env,
        }
    }

    fn get_memory(instance: &Instance) -> &Memory {
        instance.exports.get_memory("memory").unwrap()
    }

    fn get_function<'a>(instance: &'a Instance, function: &str) -> Result<&'a Function, ExportError> {
        instance.exports.get_function(function)
    }
}
