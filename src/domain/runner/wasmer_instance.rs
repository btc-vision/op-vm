use std::sync::Arc;

use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use wasmer::{CompilerConfig, ExportError, Function, FunctionEnv, FunctionEnvMut, imports, Instance, Memory, MemoryAccessError, MemoryView, Module, RuntimeError, Store, Value};
use wasmer::sys::{BaseTunables, EngineBuilder};
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints, set_remaining_points};
use wasmer_middlewares::Metering;
use wasmer_types::{FunctionType, Target, Type};

use crate::domain::contract::{AbortData, CustomEnv};
use crate::domain::runner::RunnerInstance;
use crate::domain::vm::{get_op_cost, LimitingTunables};

pub struct WasmerInstance {
    store: Store,
    instance: Instance,
    env: FunctionEnv<CustomEnv>,
}

impl WasmerInstance {
    pub fn new(bytecode: &[u8], max_gas: u64, load_function: ThreadsafeFunction<Vec<u8>, ErrorStrategy::CalleeHandled>) -> anyhow::Result<Self> {
        let metering = Arc::new(Metering::new(max_gas, get_op_cost));

        let mut compiler = Singlepass::default();
        compiler.canonicalize_nans(true);
        compiler.push_middleware(metering);
        compiler.enable_verifier();

        let base = BaseTunables::for_target(&Target::default());
        let tunables = LimitingTunables::new(base, 16, 1024 * 1024);

        let mut engine = EngineBuilder::new(compiler).set_features(None).engine();
        engine.set_tunables(tunables);

        let mut store = Store::new(engine);

        let env = FunctionEnv::new(&mut store, CustomEnv {
            memory: None,
            abort_data: None,
            load_function,
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

        /*fn load(mut env: FunctionEnvMut<CustomEnv>, pointer: u32) -> Result<u32, RuntimeError> {
            let data = env.data_mut();
            Ok((data.load_function)(pointer))
        }*/

        let deploy_from_address_mut = move |context: FunctionEnvMut<CustomEnv>, values: &[Value]| {
            let ptr: u32 = values[0].unwrap_i32() as u32;

            let async_context = Arc::new(Mutex::new(context));

            let deploy = {
                let async_context = async_context.clone();
                async move {
                    let mut context = async_context.lock().await;
                    let mutable_context = context.as_mut();
                    let state = mutable_context.data();

                    let load_func: ThreadsafeFunction<Vec<u8>> = state.load_function.clone();

                    println!("Calling load function from async context...");

                    let (env, store) = context.data_and_store_mut();
                    let memory = env.memory.clone().unwrap();
                    let view = memory.view(&store);

                    let key = read_memory(&view, ptr as u64, 32).unwrap();

                    let response: Result<Vec<u8>, RuntimeError> = load_func.call_async(Ok(key)).await.map_err(|e| {
                        println!("Error calling load function: {:?}", e);
                        RuntimeError::new("Error calling load function")
                    });

                    let data = response.unwrap();

                    println!("Data received: {:?}", &data);

                    write_memory(&view, 0, &data).unwrap();

                    Ok(vec![Value::I32(0)])
                }
            };

            let rt = Runtime::new().unwrap();
            let response = rt.block_on(deploy);

            response
        };

        let abort_typed = Function::new_typed_with_env(&mut store, &env, abort);
        let deploy_from_address_signature = FunctionType::new(
            vec![Type::I32],
            vec![Type::I32],
        );

        let deploy_from_address = Function::new_with_env(
            &mut store,
            &env,
            deploy_from_address_signature,
            deploy_from_address_mut,
        );

        let import_object = imports! {
            "env" => {
                "abort" => abort_typed,
                "deployFromAddress" => deploy_from_address,
            }
        };

        let module: Module = Module::new(&store, &bytecode)?;
        let instance: Instance = Instance::new(&mut store, &module, &import_object)?;

        env.as_mut(&mut store).memory = Some(Self::get_memory(&instance).clone());

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

fn read_memory(view: &MemoryView, offset: u64, length: u64) -> Result<Vec<u8>, RuntimeError> {

    let mut buffer: Vec<u8> = vec![0; length as usize];
    view.read(offset, &mut buffer).unwrap();

    Ok(buffer)
}

fn write_memory(view: &MemoryView, offset: u64, data: &[u8]) -> Result<(), MemoryAccessError> {
    view.write(offset, data)
}
