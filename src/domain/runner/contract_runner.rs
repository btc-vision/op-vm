use wasmer::{MemoryAccessError, Value};

use crate::domain::contract::AbortData;

pub trait ContractRunner: Send + Sync {
    fn call(&mut self, function: &str, params: &[Value]) -> anyhow::Result<Box<[Value]>>;
    fn read_memory(&self, offset: u64, length: u64) -> Result<Vec<u8>, MemoryAccessError>;
    fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), MemoryAccessError>;
    fn write_buffer(&mut self, value: &[u8], id: i32, align: u32) -> Result<i64, napi::Error>;
    fn get_remaining_gas(&mut self) -> u64;
    fn set_remaining_gas(&mut self, gas: u64);
    fn use_gas(&mut self, gas: u64);
    fn get_abort_data(&self) -> Option<AbortData>;
}
