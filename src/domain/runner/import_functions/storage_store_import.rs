use crate::domain::runner::{CustomEnv, COLD_STORAGE_GAS_COST, WARM_STORAGE_GAS_COST};
use crate::interfaces::ExternalFunction;
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct StorageStoreImport;

impl StorageStoreImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        key_ptr: u32,
        value_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new(
                "Cannot save to transient storage in start function",
            ));
        }

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let key = instance
            .read_memory(&store, key_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading storage key from memory"))?;
        let value = instance
            .read_memory(&store, value_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading storage value from memory"))?;

        let resp = env.storage_store_external.execute(&[key, value].concat(), &env.runtime)?;

        let is_slot_warm = resp[0] == 1;

        let gas_cost = if is_slot_warm {
            WARM_STORAGE_GAS_COST
        } else {
            COLD_STORAGE_GAS_COST
        };

        instance.use_gas(&mut store, gas_cost);

        Ok(())
    }
}
