use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct ConsoleLogImport;

impl ConsoleLogImport {
    pub fn execute(mut context: FunctionEnvMut<CustomEnv>, ptr: u32) -> Result<(), RuntimeError> {
        let (env, store) = context.data_and_store_mut();
        let instance = &env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Memory not found"))?;

        let network = &env.network;
        if !&network.enable_debug() {
            return Err(RuntimeError::new("Contracts may not log this network"));
        }

        let data = AssemblyScript::read_buffer(&store, &instance, ptr)
            .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

        env.console_log_external.execute(&data, &env.runtime)
    }
}
