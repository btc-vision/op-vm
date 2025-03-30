use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct TransientStorageStoreImport;

const STATIC_GAS_COST: u64 = 1_000_000;

impl TransientStorageStoreImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        key_ptr: u32,
        value_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new(
                "Cannot save to storage in start function",
            ));
        }

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, STATIC_GAS_COST);

        let key = instance
            .read_memory(&store, key_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading storage key from memory"))?;
        let value = instance
            .read_memory(&store, value_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading storage value from memory"))?;
        
        env.transient_storage.set(
            key.as_slice()
                .try_into()
                .map_err(|_| RuntimeError::new(&format!("Cannot convert the key")))?,
            value
                .as_slice()
                .try_into()
                .map_err(|_| RuntimeError::new(&format!("Cannot convert the data")))?,
        );

        Ok(())
    }
}
