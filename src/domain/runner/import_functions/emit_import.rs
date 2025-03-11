use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

const STATIC_GAS_COST: u64 = 1_000_000;

#[derive(Default)]
pub struct EmitImport;

impl EmitImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        data_ptr: u32,
        data_length: u32,
    ) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new("Cannot emit event in start function"));
        }

        let instance = &env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, STATIC_GAS_COST);

        let data = instance
            .read_memory(&store, data_ptr as u64, data_length as u64)
            .map_err(|_e| RuntimeError::new("Error reading data from memory"))?;

        env.emit_external.execute(&data, &env.runtime)
    }
}
