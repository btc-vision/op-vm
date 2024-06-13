use std::sync::Arc;

use wasmer::{
    CompilerConfig, Function, FunctionEnv, FunctionEnvMut, imports, Instance, Memory,
    MemoryAccessError, MemoryView, Module, RuntimeError, Store, Value,
};
use wasmer::sys::EngineBuilder;
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints};
use wasmer_middlewares::Metering;
use wasmer_types::RawValue;

use crate::domain::assembly_script::AssemblyScript;
use crate::domain::vm::get_op_cost;

pub struct Contract {
    pub store: Store,
    pub instance: Instance,
}

const MAX_GAS: u64 = 300_000_000_000;

impl Contract {
    pub fn new(bytecode: &[u8]) -> Self {
        let metering = Arc::new(Metering::new(MAX_GAS, get_op_cost));

        let mut compiler = Singlepass::default();
        compiler.canonicalize_nans(true);
        compiler.push_middleware(metering);

        let engine = EngineBuilder::new(compiler).set_features(None).engine();
        let mut store = Store::new(engine);

        struct MyEnv;
        let env = FunctionEnv::new(&mut store, MyEnv {});
        fn abort(_env: FunctionEnvMut<MyEnv>, _: i32, _: i32, _: i32, _: i32) {
            std::process::exit(-1)
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
        }
    }

    pub fn init(&mut self, address: &str, deployer: &str) {
        let contract_address: i32 =
            AssemblyScript::lower_string(self, &address).unwrap() as i32;
        let deployer_address: i32 =
            AssemblyScript::lower_string(self, &deployer).unwrap() as i32;

        self.call(
            "INIT",
            &[Value::I32(contract_address), Value::I32(deployer_address)],
        )
            .unwrap();
    }

    pub fn read_pointer(&self, offset: u64, length: u64) -> Result<Vec<u8>, RuntimeError> {
        let memory = Self::get_memory(&self.instance);
        let view: MemoryView = memory.view(&self.store);

        let mut buffer: Vec<u8> = vec![0; length as usize];
        for i in 0..length {
            let byte = view.read_u8(offset + i);

            // check for error
            if byte.is_err() {
                return Err(RuntimeError::new("Out of bounds memory access"));
            }

            buffer[i as usize] = byte.unwrap();
        }

        Ok(buffer)
    }

    pub fn write_pointer(&mut self, offset: u64, value: Vec<u8>) -> Result<(), MemoryAccessError> {
        let memory = Self::get_memory(&self.instance);
        let view = memory.view(&mut self.store);
        return view.write(offset, &value);
    }

    pub fn set_u32(&mut self, offset: i32, value: u32) -> Result<(), MemoryAccessError> {
        let memory = Self::get_memory(&self.instance);
        let view = memory.view(&mut self.store);

        return view.write(offset as u64, &value.to_le_bytes());
    }

    pub fn read_memory(&self, offset: u64, length: u64) -> Result<Vec<u8>, RuntimeError> {
        let memory = Self::get_memory(&self.instance);
        let view = memory.view(&self.store);

        let mut buffer: Vec<u8> = vec![0; length as usize];
        view.read(offset, &mut buffer).unwrap();

        Ok(buffer)
    }

    pub fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), MemoryAccessError> {
        let memory = Self::get_memory(&self.instance);
        let view = memory.view(&self.store);
        return view.write(offset, data);
    }

    pub fn call(&mut self, function: &str, params: &[Value]) -> Result<Box<[Value]>, RuntimeError> {
        println!("Calling {function}...");
        let export = Self::get_function(&self.instance, &function);
        let response = export.call(&mut self.store, params);
        self.print_results(&response);
        response
    }

    #[allow(dead_code)]
    pub fn call_raw(
        &mut self,
        function: &str,
        params: Vec<RawValue>,
    ) -> Result<Box<[Value]>, RuntimeError> {
        println!("Calling {function}...");
        let export = Self::get_function(&self.instance, &function);
        let response = export.call_raw(&mut self.store, params);
        self.print_results(&response);
        response
    }

    fn get_memory(instance: &Instance) -> &Memory {
        instance.exports.get_memory("memory").unwrap()
    }

    fn get_function<'a>(instance: &'a Instance, function: &str) -> &'a Function {
        instance.exports.get_function(function).unwrap()
    }

    fn print_results(&mut self, response: &Result<Box<[Value]>, RuntimeError>) {
        match &response {
            Ok(results) => println!("Results: {:?}", &results),
            Err(error) => {
                println!("Execution failed");
                let remaining_points = get_remaining_points(&mut self.store, &self.instance);
                match remaining_points {
                    MeteringPoints::Remaining(_) => eprintln!("{}", &error),
                    MeteringPoints::Exhausted => (),
                };
            }
        }

        let remaining_points = get_remaining_points(&mut self.store, &self.instance);

        let gas_used = match remaining_points {
            MeteringPoints::Remaining(remaining) => MAX_GAS - remaining,
            MeteringPoints::Exhausted => MAX_GAS,
        };

        println!("Gas used: {gas_used}/{MAX_GAS}");
    }
}
