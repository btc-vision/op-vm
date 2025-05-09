use wasmer::Value;

use crate::domain::runner::{EnvironmentVariables, ExitData, ExtendedMemoryAccessError};

pub trait ContractRunner: Send + Sync {
    fn set_environment_variables(&mut self, environment_variables: EnvironmentVariables);
    fn on_deploy(&mut self, calldata: Vec<u8>, max_gas: u64) -> anyhow::Result<ExitData>;
    fn execute(&mut self, calldata: Vec<u8>, max_gas: u64) -> anyhow::Result<ExitData>;
    fn call_export_by_name(
        &mut self,
        function: &str,
        params: &[Value],
        max_gas: u64,
    ) -> anyhow::Result<Box<[Value]>>;
    fn read_memory(&self, offset: u64, length: u64) -> Result<Vec<u8>, ExtendedMemoryAccessError>;
    fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), ExtendedMemoryAccessError>;
    fn get_remaining_gas(&mut self) -> u64;
    fn get_used_gas(&mut self) -> u64;
    fn is_out_of_memory(&self) -> Result<bool, ExtendedMemoryAccessError>;
    fn set_remaining_gas(&mut self, gas: u64);
    fn use_gas(&mut self, gas: u64);
    fn get_exit_data(&self) -> ExitData;
}
