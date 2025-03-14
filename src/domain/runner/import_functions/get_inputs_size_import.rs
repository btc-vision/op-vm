use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

const STATIC_GAS_COST: u64 = 20_000;

#[derive(Default)]
pub struct GetInputsSizeImport;

impl GetInputsSizeImport {
    pub fn execute(mut context: FunctionEnvMut<CustomEnv>) -> Result<u32, RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new("Cannot get inputs size in start function"));
        }

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, STATIC_GAS_COST);

        // TODO: Don't load all inputs for this
        let inputs = &env.inputs_external.execute(&env.runtime)?;

        Ok(inputs.len() as u32)
    }
}
