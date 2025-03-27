use crate::domain::runner::CustomEnv;
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
                "Cannot save to storage in start function",
            ));
        }

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;
        let mut cache = env
            .store_cache
            .lock()
            .map_err(|e| RuntimeError::new(format!("Error claiming store cache: {}", e)))?;

        let key = instance
            .read_memory(&store, key_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading storage key from memory"))?;
        let value = instance
            .read_memory(&store, value_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading storage value from memory"))?;

        let data = [key.as_slice(), value.as_slice()].concat();

        let pointer = data
            .get(0..32)
            .ok_or(RuntimeError::new("Invalid buffer"))?
            .to_vec();
        let value = data
            .get(32..64)
            .ok_or(RuntimeError::new("Invalid buffer"))?
            .to_vec();

        let result = cache.set(
            pointer
                .try_into()
                .map_err(|e| RuntimeError::new(format!("Cannot convert the pointer: {:?}", e)))?,
            value
                .try_into()
                .map_err(|e| RuntimeError::new(format!("Cannot convert the pointer: {:?}", e)))?,
            |key| {
                let resp = env.storage_load_external.execute(&key, &env.runtime)?;

                let is_cold = resp[32] == 1;
                let pointer_value = resp[0..32].try_into().map_err(|e| {
                    RuntimeError::new(format!("Cannot map result to data: {:?}", e))
                })?;

                Ok((pointer_value, is_cold))
            },
            |key, value| {
                env.storage_store_external
                    .execute(
                        &key.iter().chain(value.iter()).cloned().collect::<Vec<u8>>(),
                        &env.runtime,
                    )
                    .map_err(|e| {
                        RuntimeError::new(format!("Cannot map result to data: {:?}", e))
                    })?;
                Ok(())
            },
        )?;

        instance.use_gas(&mut store, result.gas_cost);
        if result.gas_refund > 0 {
            instance.refund_gas(&mut store, result.gas_refund as u64);
        } else if result.gas_refund < 0 && result.gas_refund > i64::MIN {
            instance.use_gas(&mut store, (-result.gas_refund) as u64);
        } else if result.gas_refund == i64::MIN {
            instance.use_gas(&mut store, u64::MAX);
        }

        Ok(())
    }
}
