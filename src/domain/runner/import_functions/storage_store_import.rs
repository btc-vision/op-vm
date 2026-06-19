use crate::domain::runner::{
    CustomEnv, HardFork, COLD_STORAGE_GAS_COST_RACHEL, WARM_STORAGE_GAS_COST_RACHEL,
};
use wasmer::{FunctionEnvMut, RuntimeError};

const STORAGE_STORE_GAS_COST_ROSWELL: u64 = 1_000_000;

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

        let resp = env
            .storage_store_external
            .execute(&[key, value].concat(), &env.runtime)?;

        let is_slot_warm = resp[0] == 1;

        let gas_cost: u64 = match env.hard_fork {
            HardFork::Roswell => STORAGE_STORE_GAS_COST_ROSWELL,
            HardFork::Rachel => {
                if is_slot_warm {
                    WARM_STORAGE_GAS_COST_RACHEL
                } else {
                    COLD_STORAGE_GAS_COST_RACHEL
                }
            }
        };

        env.charge_gas(&instance, &mut store, gas_cost)?;

        Ok(())
    }
}
