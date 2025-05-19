use crate::domain::runner::{CustomEnv, COLD_STORAGE_GAS_COST, WARM_STORAGE_GAS_COST};
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

        let key = instance
            .read_memory(&store, key_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading storage key from memory"))?;

        let response = env.storage_load_external.execute(&key, &env.runtime)?;
        let value = response[0..32]
            .try_into()
            .map_err(|e| RuntimeError::new(format!("Cannot map result to data: {:?}", e)))?;

        let is_slot_warm = response[32] == 1;

        let gas_cost = if is_slot_warm {
            WARM_STORAGE_GAS_COST
        } else {
            COLD_STORAGE_GAS_COST
        };

        instance.use_gas(&mut store, gas_cost);

        instance
            .write_memory(&store, result_ptr as u64, value)
            .map_err(|_e| RuntimeError::new("Error writing storage value to memory"))?;

        Ok(())
    }
}
