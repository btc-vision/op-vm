use std::sync::Arc;

use anyhow::anyhow;
use napi::Error;
use wasmer::Value;

use crate::domain::runner::{AbortData, ExtendedMemoryAccessError, ThreadedWasmerRunner};

/// A service that wraps high-level contract operations.
/// Now holds an Arc<ThreadedWasmerRunner> instead of Arc<Mutex<dyn ContractRunner>>.
pub struct ContractService {
    /// The total gas limit for this contract
    max_gas: u64,

    /// The concurrency-safe runner on a dedicated thread
    runner: Arc<ThreadedWasmerRunner>,

    /// An internal "is executing" flag that you can use however you like
    is_executing: bool,
}

impl ContractService {
    /// Create a new ContractService with the given `max_gas` and concurrency runner
    pub fn new(max_gas: u64, runner: Arc<ThreadedWasmerRunner>) -> Self {
        Self {
            max_gas,
            runner,
            is_executing: false,
        }
    }

    /// Toggle an "is executing" flag if your flow needs it
    pub fn set_executing(&mut self, executing: bool) {
        self.is_executing = executing;
    }

    pub fn is_executing(&self) -> bool {
        self.is_executing
    }

    /// Call a function exported by the contract.
    /// Automatically checks for out-of-gas or out-of-memory.
    pub fn call(&mut self, function: &str, params: &[Value]) -> anyhow::Result<Box<[Value]>> {
        //println!("!!! [call] Calling function: {:?} !!!", function);
        let response = self.runner.call(function, params).map_err(|e| {
            // Handle typical WASM "unreachable" traps, often caused by out-of-gas or out-of-memory.
            if e.to_string().contains("unreachable") {
                let remaining_gas = self.runner.get_remaining_gas();
                let gas_used = self.max_gas - remaining_gas;

                if remaining_gas == 0 {
                    // No gas left
                    anyhow!("out of gas (consumed: {gas_used})")
                } else {
                    // Possibly out-of-memory?
                    let out_of_memory = self.runner.is_out_of_memory().unwrap_or(false);
                    if out_of_memory {
                        anyhow!("out of memory")
                    } else {
                        anyhow!(e)
                    }
                }
            } else {
                anyhow!(e)
            }
        });

        response
    }

    /// Returns how much gas has been consumed so far
    pub fn get_used_gas(&mut self) -> u64 {
        let remaining_gas = self.get_remaining_gas();
        let gas_used = self.max_gas - remaining_gas;
        gas_used
    }

    /// Manually set how much gas has been consumed
    /// (by setting `remaining_gas = max_gas - used`).
    pub fn set_used_gas(&mut self, gas_used: u64) {
        let remaining = self.max_gas.saturating_sub(gas_used);
        self.set_remaining_gas(remaining);
    }

    /// Read the contract's currently remaining gas
    pub fn get_remaining_gas(&mut self) -> u64 {
        self.runner.get_remaining_gas()
    }

    /// Directly set how much gas remains.
    /// Be sure to pass the actual "remaining" if you want to do `max_gas - used`.
    pub fn set_remaining_gas(&mut self, remaining: u64) {
        self.runner.set_remaining_gas(remaining);
    }

    /// Use (subtract) some amount of gas
    pub fn use_gas(&mut self, gas: u64) {
        self.runner.use_gas(gas);
    }

    /// Read memory from the WASM instance
    pub fn read_memory(
        &self,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, ExtendedMemoryAccessError> {
        self.runner.read_memory(offset, length)
    }

    /// Write memory into the WASM instance
    pub fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), ExtendedMemoryAccessError> {
        self.runner.write_memory(offset, data)
    }

    /// Write a buffer according to the AssemblyScript convention
    pub fn write_buffer(&mut self, value: &[u8], id: i32, align: u32) -> Result<i64, Error> {
        self.runner.write_buffer(value, id, align)
    }

    /// Returns any stored "abort" debug data from inside the WASM
    pub fn get_abort_data(&self) -> Option<AbortData> {
        self.runner.get_abort_data()
    }

    /// Optional helper: Print call results, gas usage, etc.
    #[allow(dead_code)]
    fn print_results(&mut self, response: &anyhow::Result<Box<[Value]>>) {
        let remaining_gas = self.runner.get_remaining_gas();

        match &response {
            Ok(results) => println!("Results: {:?}", results),
            Err(error) => {
                println!("Execution failed");
                // If there's still gas left, show the error, otherwise it's out-of-gas
                if remaining_gas > 0 {
                    eprintln!("{}", &error);
                }
            }
        }

        let gas_used = self.max_gas - remaining_gas;
        println!("Gas used: {gas_used}/{}", self.max_gas);
    }
}
