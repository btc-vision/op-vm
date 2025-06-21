use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

const STATIC_GAS_COST: u64 = 30_000;
const GAS_COST_PER_BYTE: u64 = 1_000;

#[derive(Default)]
pub struct GetInputsImport;

impl GetInputsImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new("Cannot get inputs in start function"));
        }

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, STATIC_GAS_COST);

        let result = &env.inputs_external.execute_empty_request(&env.runtime)?;

        instance.use_gas(&mut store, result.len() as u64 * GAS_COST_PER_BYTE);

        instance
            .write_memory(&store, result_ptr as u64, result)
            .map_err(|_e| RuntimeError::new("Error writing inputs to memory"))?;

        Ok(())
    }
}
