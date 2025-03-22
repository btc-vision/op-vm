use crate::domain::runner::CustomEnv;
use crate::interfaces::ExternalFunction;
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct StorageLoadImport;

impl StorageLoadImport {
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

        let mut cache = env
            .store_cache
            .lock()
            .map_err(|e| RuntimeError::new(format!("Error claiming store cache: {}", e)))?;

        // Get method
        let result = cache.get(
            &data
                .try_into()
                .map_err(|e| RuntimeError::new(format!("Cannot convert the pointer: {:?}", e)))?,
            |key| {
                Ok(env
                    .storage_load_external
                    .execute(&key, &env.runtime)?
                    .try_into()
                    .map_err(|e| {
                        RuntimeError::new(format!("Cannot map result to data: {:?}", e))
                    })?)
            },
        )?;

        instance.use_gas(&mut store, result.gas_cost);

        instance
            .write_memory(&store, result_ptr as u64, &result.value)
            .map_err(|_e| RuntimeError::new("Error writing storage value to memory"))?;

        Ok(())
    }
}
