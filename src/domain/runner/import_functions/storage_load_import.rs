use crate::domain::runner::{
    CustomEnv, HardFork, COLD_STORAGE_GAS_COST_RACHEL, WARM_STORAGE_GAS_COST_RACHEL,
};
use wasmer::{FunctionEnvMut, RuntimeError};

const COLD_STORAGE_LOAD_GAS_COST_ROSWELL: u64 = 1_000_000;
const WARM_STORAGE_LOAD_GAS_COST_ROSWELL: u64 = 21_000_000;

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

        let gas_cost: u64 = match env.hard_fork {
            HardFork::Roswell => {
                if is_slot_warm {
                    WARM_STORAGE_LOAD_GAS_COST_ROSWELL
                } else {
                    COLD_STORAGE_LOAD_GAS_COST_ROSWELL
                }
            }
            HardFork::Rachel => {
                if is_slot_warm {
                    WARM_STORAGE_GAS_COST_RACHEL
                } else {
                    COLD_STORAGE_GAS_COST_RACHEL
                }
            }
        };

        env.charge_gas(&instance, &mut store, gas_cost)?;

        instance
            .write_memory(&store, result_ptr as u64, value)
            .map_err(|_e| RuntimeError::new("Error writing storage value to memory"))?;

        Ok(())
    }
}
