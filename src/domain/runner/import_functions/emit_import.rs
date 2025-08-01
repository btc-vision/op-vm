use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

const STATIC_GAS_COST: u64 = 3_750_000;
const GAS_COST_PER_BYTES: u64 = 80_000;

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

        instance.use_gas(&mut store, data.len() as u64 * GAS_COST_PER_BYTES);

        env.emit_external.execute_no_response(&data, &env.runtime)
    }
}
