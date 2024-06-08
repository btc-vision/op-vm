use std::sync::Arc;

use wasmer::{
    CompilerConfig, Function, FunctionEnv, FunctionEnvMut, imports, Instance, Module, RuntimeError,
    Store, Value,
};
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
    pub fn new(bytecode: Vec<u8>, address: &str, deployer: &str) -> Self {
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
            bytecode,
            address: address.to_string(),
            deployer: deployer.to_string(),
            store,
            instance,
        }
    }

    pub fn init(&mut self) -> Result<Box<[Value]>, RuntimeError> {
        let __new = self.instance.exports.get_function("__new").unwrap();
        let __pin = self.instance.exports.get_function("__pin").unwrap();
        let memory = self.instance.exports.get_memory("memory").unwrap();

        let contract_address: i32 =
            lower_string(&mut self.store, &self.address, __new, __pin, memory).unwrap() as i32;
        let deployer_address: i32 =
            lower_string(&mut self.store, &self.deployer, __new, __pin, memory).unwrap() as i32;

        self.call_wasm_function(
            "INIT",
            &[Value::I32(contract_address), Value::I32(deployer_address)],
        )
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
