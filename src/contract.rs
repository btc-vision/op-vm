use std::sync::Arc;

use anyhow::anyhow;
use wasmer::{CompilerConfig, Function, FunctionEnv, FunctionEnvMut, imports, Instance, Memory, MemoryAccessError, MemoryView, Module, RuntimeError, Store, Value};
use wasmer::sys::EngineBuilder;
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints};
use wasmer_middlewares::Metering;
use wasmer_types::RawValue;

use crate::vm::{get_op_cost, lower_string};

pub struct Contract {
    pub bytecode: Vec<u8>,
    pub address: String,
    pub deployer: String,
    pub store: Store,
    pub instance: Instance,
}

const MAX_GAS: u64 = 300_000_000_000;

impl Contract {
    pub fn new(bytecode: Vec<u8>, address: &str, deployer: &str) -> Result<Self, String> {
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

        Ok(Self {
            bytecode,
            address: address.to_string(),
            deployer: deployer.to_string(),
            store,
            instance,
        })
    }

    pub fn get_memory(&self) -> &Memory {
        return self.instance.exports.get_memory("memory").unwrap();
    }

    pub fn __new(&mut self, length: i32, id: i32) -> anyhow::Result<i32> {
        let result = self.call_wasm_function(
            "__new",
            &[Value::I32(length << 1), Value::I32(id)],
        )?;

        let pointer = result
            .get(0)
            .ok_or(anyhow!("can't get pointer"))?
            .i32()
            .ok_or(anyhow!("can't get pointer"))?;

        return Ok(pointer);
    }

    pub fn lower_string(&mut self, value: &str) -> anyhow::Result<u32> {
        let new = self.instance.exports.get_function("__new").unwrap();
        let pin = self.instance.exports.get_function("__pin").unwrap();
        let memory = self.instance.exports.get_memory("memory").unwrap();

        lower_string(&mut self.store, value, &new, &pin, &memory)
    }

    pub fn init(&mut self) -> Result<Box<[Value]>, RuntimeError> {
        let new = self.instance.exports.get_function("__new").unwrap();
        let pin = self.instance.exports.get_function("__pin").unwrap();
        let memory = self.instance.exports.get_memory("memory").unwrap();

        let contract_address: i32 =
            lower_string(&mut self.store, &self.address, &new, &pin, &memory).unwrap() as i32;
        let deployer_address: i32 =
            lower_string(&mut self.store, &self.deployer, &new, &pin, &memory).unwrap() as i32;

        self.call_wasm_function(
            "INIT",
            &[Value::I32(contract_address), Value::I32(deployer_address)],
        )
    }

    pub fn ping(&mut self) -> Result<Box<[Value]>, RuntimeError> {
        self.call_wasm_function("ping", &[])
    }

    pub fn write_pointer(&mut self, offset: u64, value: Vec<u8>) -> Result<(), MemoryAccessError> {
        let memory = self.instance.exports.get_memory("memory").unwrap();
        let view = memory.view(&mut self.store);
        return view.write(offset, &value);
    }

    pub fn set_u32(&mut self, offset: i32, value: u32) -> Result<(), MemoryAccessError> {
        let memory = self.instance.exports.get_memory("memory").unwrap();
        let view = memory.view(&mut self.store);

        return view.write(offset as u64, &value.to_le_bytes());
    }

    pub fn __unpin(&mut self, pointer: i32) -> Result<Box<[Value]>, RuntimeError> {
        self.call_wasm_function("__unpin", &[Value::I32(pointer)])
    }

    pub fn __pin(&mut self, pointer: i32) -> Result<Box<[Value]>, RuntimeError> {
        self.call_wasm_function("__pin", &[Value::I32(pointer)])
    }

    pub fn read_pointer(&mut self, offset: u64, length: u64) -> Result<Vec<u8>, RuntimeError> {
        let memory = self.instance.exports.get_memory("memory").unwrap();
        let view: MemoryView = memory.view(&mut self.store);

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

    pub fn read(&mut self, offset: u64, length: u64) -> Result<Vec<u8>, RuntimeError> {
        let memory = self.instance.exports.get_memory("memory").unwrap();
        let view: MemoryView = memory.view(&mut self.store);

        let mut buffer: Vec<u8> = vec![0; length as usize];
        view.read(offset, &mut buffer).unwrap();

        Ok(buffer)
    }

    pub fn write(&mut self, offset: u64, data: &Vec<u8>) -> Result<(), MemoryAccessError> {
        let memory = self.instance.exports.get_memory("memory").unwrap();
        let view = memory.view(&mut self.store);
        return view.write(offset, data);
    }

    pub fn call(&mut self, function: &str, params: &[Value]) -> Result<Box<[Value]>, RuntimeError> {
        self.call_wasm_function(function, params)
    }

    pub fn call_raw(&mut self, function: &str, params: Vec<RawValue>) -> Result<Box<[Value]>, RuntimeError> {
        self.call_wasm_function_raw(function, params)
    }

    fn call_wasm_function(
        &mut self,
        function: &str,
        params: &[Value],
    ) -> Result<Box<[Value]>, RuntimeError> {
        println!("Calling {function}...");
        let export = self.instance.exports.get_function(function).unwrap();
        let response = export.call(&mut self.store, params);
        self.print_results(&response);
        response
    }

    fn call_wasm_function_raw(
        &mut self,
        function: &str,
        params: Vec<RawValue>,
    ) -> Result<Box<[Value]>, RuntimeError> {
        println!("Calling {function}...");
        let export = self.instance.exports.get_function(function).unwrap();
        let response = export.call_raw(&mut self.store, params);
        self.print_results(&response);
        response
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
