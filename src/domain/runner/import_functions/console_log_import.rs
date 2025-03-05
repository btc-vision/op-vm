use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct ConsoleLogImport;

impl ConsoleLogImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        data_ptr: u32,
        data_length: u32,
    ) -> Result<(), RuntimeError> {
        let (env, store) = context.data_and_store_mut();
        let instance = &env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Memory not found"))?;

        let data = instance
            .read_memory(&store, data_ptr as u64, data_length as u64)
            .map_err(|_e| RuntimeError::new("Error reading data from memory"))?;

        env.console_log_external.execute(&data, &env.runtime)
    }
}
