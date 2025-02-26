use std::sync::{Arc, Mutex};

use napi::Error;
use wasmer::Value;

use crate::domain::runner::{ContractRunner, ExtendedMemoryAccessError, RevertData};

pub struct ContractService {
    max_gas: u64,
    runner: Arc<Mutex<dyn ContractRunner>>,
}

impl ContractService {
    pub fn new(max_gas: u64, runner: Arc<Mutex<dyn ContractRunner>>) -> Self {
        Self { max_gas, runner }
    }

    pub fn call(&mut self, function: &str, params: &[Value]) -> anyhow::Result<Box<[Value]>> {
        let mut runner = self.runner.lock().map_err(|_| anyhow::anyhow!("Failed to lock runner"))?;
        let response = runner.call(function, params).map_err(|e| {
            if e.to_string().contains("unreachable") {
                let gas_used = runner.get_remaining_gas();
                if gas_used == 0 {
                    anyhow::anyhow!(
                        "out of gas (consumed: {})",
                        self.max_gas
                    )
                } else {
                    let out_of_memory = runner.is_out_of_memory().unwrap_or(false);

                    if out_of_memory {
                        anyhow::anyhow!("out of memory")
                    } else {
                        anyhow::anyhow!(e)
                    }
                }
            } else {
                anyhow::anyhow!(e)
            }
        });

        response
    }

    pub fn get_used_gas(&mut self) -> u64 {
        let remaining_gas = self.get_remaining_gas();
        let gas_used = self.max_gas - remaining_gas;

        gas_used
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

    pub fn read_memory(&self, offset: u64, length: u64) -> Result<Vec<u8>, ExtendedMemoryAccessError> {
        let runner = self.runner.lock().unwrap();
        runner.read_memory(offset, length)
    }

    pub fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), ExtendedMemoryAccessError> {
        let runner = self.runner.lock().unwrap();
        runner.write_memory(offset, data)
    }

    pub fn write_buffer(&mut self, value: &[u8], id: i32, align: u32) -> Result<i64, Error> {
        let mut runner = self.runner.lock().unwrap();
        runner.write_buffer(value, id, align)
    }

    pub fn get_revert_data(&self) -> Option<RevertData> {
        let runner = self.runner.lock().unwrap();
        runner.get_revert_data()
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
