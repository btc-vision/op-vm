use napi::Error;
use wasmer::{MemoryAccessError, RuntimeError, Value};
use wasmer_types::RawValue;

use crate::domain::assembly_script::AssemblyScript;
use crate::domain::contract::AbortData;
use crate::domain::runner::RunnerInstance;
use crate::domain::vm::MAX_GAS;

pub struct Contract {
    runner: Box<dyn RunnerInstance>,
}

impl Contract {
    pub fn new(runner: Box<dyn RunnerInstance>) -> Self {
        Self { runner }
    }

    pub fn call(&mut self, function: &str, params: &[Value]) -> anyhow::Result<Box<[Value]>> {
        println!("Calling {function}...");
        let response = self.runner.call(&function, params);
        self.print_results(&response);
        response
    }

    pub fn call_raw(
        &mut self,
        function: &str,
        params: Vec<RawValue>,
    ) -> anyhow::Result<Box<[Value]>> {
        println!("Calling {function}...");
        let response = self.runner.call_raw(&function, &params);
        self.print_results(&response);
        response
    }

    pub fn read_memory(&self, offset: u64, length: u64) -> Result<Vec<u8>, RuntimeError> {
        self.runner.read_memory(offset, length)
    }

    pub fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), MemoryAccessError> {
        self.runner.write_memory(offset, data)
    }

    pub fn write_buffer(
        &mut self,
        value: &[u8],
        id: i32,
        align: u32,
    ) -> Result<i64, Error> {
        AssemblyScript::write_buffer(&mut self.runner, value, id, align)
    }

    pub fn get_abort_data(&self) -> Option<AbortData> {
        self.runner.get_abort_data()
    }

    fn print_results(&mut self, response: &anyhow::Result<Box<[Value]>>) {
        let remaining_gas = self.runner.get_remaining_gas();

        match &response {
            Ok(results) => println!("Results: {:?}", &results),
            Err(error) => {
                println!("Execution failed");
                match remaining_gas {
                    0 => (),
                    _ => eprintln!("{}", &error),
                };
            }
        }

        let gas_used = MAX_GAS - remaining_gas;
        println!("Gas used: {gas_used}/{MAX_GAS}");
    }
}
