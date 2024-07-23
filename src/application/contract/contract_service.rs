use std::sync::{Arc, Mutex};

use napi::Error;
use wasmer::{MemoryAccessError, Value};

use crate::domain::runner::{AbortData, ContractRunner};

pub struct ContractService {
    max_gas: u64,
    runner: Arc<Mutex<dyn ContractRunner>>,
}

impl ContractService {
    pub fn new(max_gas: u64, runner: Arc<Mutex<dyn ContractRunner>>) -> Self {
        Self { max_gas, runner }
    }

    pub fn call(&mut self, function: &str, params: &[Value]) -> anyhow::Result<Box<[Value]>> {
        let mut runner = self.runner.lock().unwrap();
        let response = runner.call(function, params);
        response
    }

    pub fn get_used_gas(&mut self) -> u64 {
        self.max_gas - self.get_remaining_gas()
    }

    pub fn set_used_gas(&mut self, gas: u64) {
        self.set_remaining_gas(self.max_gas - gas);
    }

    pub fn get_remaining_gas(&mut self) -> u64 {
        let mut runner = self.runner.lock().unwrap();
        runner.get_remaining_gas()
    }

    pub fn set_remaining_gas(&mut self, gas: u64) {
        let mut runner = self.runner.lock().unwrap();
        runner.set_remaining_gas(self.max_gas - gas);
    }

    pub fn use_gas(&mut self, gas: u64) {
        let mut runner = self.runner.lock().unwrap();
        runner.use_gas(gas);
    }

    pub fn read_memory(&self, offset: u64, length: u64) -> Result<Vec<u8>, MemoryAccessError> {
        let runner = self.runner.lock().unwrap();
        runner.read_memory(offset, length)
    }

    pub fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), MemoryAccessError> {
        let runner = self.runner.lock().unwrap();
        runner.write_memory(offset, data)
    }

    pub fn write_buffer(&mut self, value: &[u8], id: i32, align: u32) -> Result<i64, Error> {
        let mut runner = self.runner.lock().unwrap();
        runner.write_buffer(value, id, align)
    }

    pub fn get_abort_data(&self) -> Option<AbortData> {
        let runner = self.runner.lock().unwrap();
        runner.get_abort_data()
    }

    #[allow(dead_code)]
    fn print_results(&mut self, response: &anyhow::Result<Box<[Value]>>) {
        let mut runner = self.runner.lock().unwrap();
        let remaining_gas = runner.get_remaining_gas();

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

        let max_gas = self.max_gas;
        let gas_used = max_gas - remaining_gas;
        println!("Gas used: {gas_used}/{max_gas}");
    }
}