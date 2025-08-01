use std::sync::{Arc, Mutex};

use wasmer::Value;

use crate::domain::runner::{ContractRunner, EnvironmentVariables, ExitData};

pub struct ContractService {
    max_gas: u64,
    runner: Arc<Mutex<dyn ContractRunner>>,
}

impl ContractService {
    pub fn new(max_gas: u64, runner: Arc<Mutex<dyn ContractRunner>>) -> Self {
        Self { max_gas, runner }
    }

    pub fn set_environment_variables(
        &mut self,
        environment_variables: EnvironmentVariables,
    ) -> anyhow::Result<()> {
        let mut runner = self
            .runner
            .lock()
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;
        runner.set_environment_variables(environment_variables);

        Ok(())
    }

    pub fn on_deploy(&mut self, calldata: Vec<u8>) -> anyhow::Result<ExitData> {
        let mut runner = self
            .runner
            .lock()
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;
        runner.on_deploy(calldata, self.max_gas)
    }

    pub fn execute(&mut self, calldata: Vec<u8>) -> anyhow::Result<ExitData> {
        let mut runner = self
            .runner
            .lock()
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;
        runner.execute(calldata, self.max_gas)
    }

    /*pub fn call_export_by_name(
        &mut self,
        function_name: &str,
        params: &[Value],
    ) -> anyhow::Result<Box<[Value]>> {
        let mut runner = self
            .runner
            .lock()
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;
        let response = runner
            .call_export_by_name(function_name, params, self.max_gas)
            .map_err(|e| {
                if e.to_string().contains("unreachable") {
                    let gas_used = runner.get_remaining_gas();
                    if gas_used == 0 {
                        anyhow::anyhow!("out of gas (consumed: {})", self.max_gas)
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
    }*/

    pub fn get_used_gas(&mut self) -> anyhow::Result<u64> {
        let mut runner = self
            .runner
            .lock()
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;
        Ok(runner.get_used_gas())
    }

    pub fn use_gas(&mut self, gas: u64) -> anyhow::Result<()> {
        let mut runner = self
            .runner
            .lock()
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;
        Ok(runner.use_gas(gas))
    }

    pub fn read_memory(&self, offset: u64, length: u64) -> anyhow::Result<Vec<u8>> {
        let runner = self
            .runner
            .lock()
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;
        runner
            .read_memory(offset, length)
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))
    }

    pub fn write_memory(&self, offset: u64, data: &[u8]) -> anyhow::Result<()> {
        let runner = self
            .runner
            .lock()
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;

        runner
            .write_memory(offset, data)
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))
    }

    pub fn get_exit_data(&self) -> anyhow::Result<ExitData> {
        let runner = self
            .runner
            .lock()
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;
        Ok(runner.get_exit_data())
    }

    #[allow(dead_code)]
    fn print_results(&mut self, response: &anyhow::Result<Box<[Value]>>) -> anyhow::Result<()> {
        let mut runner = self
            .runner
            .lock()
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;
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

        Ok(())
    }
}
