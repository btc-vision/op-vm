use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

pub const STATIC_GAS_COST: u64 = 5_000_000;

#[derive(Default)]
pub struct OutputsImport;

impl OutputsImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new("Cannot get outputs in start function"));
        }
        
        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, STATIC_GAS_COST);

        let result = &env.outputs_external.execute(&env.runtime)?;

        instance
            .write_memory(&store, result_ptr as u64, result)
            .map_err(|_e| RuntimeError::new("Error writing call result to memory"))?;

        Ok(())
    }
}
