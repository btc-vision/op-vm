use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct TransientStorageLoadImport;

const TLOAD_GAS_COST: u64 = 1_000_000;

impl TransientStorageLoadImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        key_ptr: u32,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new(
                "Cannot load from storage in start function",
            ));
        }

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let data = instance
            .read_memory(&store, key_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading storage key from memory"))?;

        let tstore = env
            .transient_storage
            .lock()
            .map_err(|e| RuntimeError::new(format!("Error claiming store cache: {}", e)))?;

        instance.use_gas(&mut store, TLOAD_GAS_COST);
        // Get method
        if let Some(result) = tstore.get(&data) {
            instance
                .write_memory(&store, result_ptr as u64, result)
                .map_err(|_e| RuntimeError::new("Error writing storage value to memory"))?;
        } else {
            instance
                .write_memory(&store, result_ptr as u64, &[0; 32])
                .map_err(|_e| RuntimeError::new("Error writing storage value to memory"))?;
        }

        Ok(())
    }
}
