use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct TransientLoadImport;

const STATIC_GAS_COST: u64 = 1_000_000;

impl TransientLoadImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        key_ptr: u32,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new(
                "Cannot load from transient storage in start function",
            ));
        }

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, STATIC_GAS_COST);
        
        let data = instance
            .read_memory(&store, key_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading storage key from memory"))?;

        // Get method
        let result = env
            .transient_storage
            .get(data.as_slice().try_into().unwrap());

        instance
            .write_memory(&store, result_ptr as u64, &result)
            .map_err(|_e| RuntimeError::new("Error writing storage value to memory"))?;

        Ok(())
    }
}
