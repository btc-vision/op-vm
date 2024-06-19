use wasmer::{MemoryAccessError, RuntimeError, Value};
use wasmer_types::RawValue;

use crate::domain::contract::AbortData;

pub trait RunnerInstance {
    fn call(&mut self, function: &str, params: &[Value]) -> anyhow::Result<Box<[Value]>>;
    fn call_raw(&mut self, function: &str, params: &[RawValue]) -> anyhow::Result<Box<[Value]>>;
    fn read_memory(&self, offset: u64, length: u64) -> Result<Vec<u8>, RuntimeError>;
    fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), MemoryAccessError>;
    fn get_remaining_gas(&mut self) -> u64;
    fn get_abort_data(&self) -> Option<AbortData>;
}