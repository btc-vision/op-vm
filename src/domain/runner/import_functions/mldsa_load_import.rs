use crate::domain::runner::{CustomEnv, LOAD_MLDSA_PUBLIC_KEY_GAS_COST};
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct MLDSALoadImport;

impl MLDSALoadImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        key_ptr: u32,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new("Cannot load mldsa in start function"));
        }

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let key = instance
            .read_memory(&store, key_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading mldsa from memory"))?;

        let response = env.mldsa_load_external.execute(&key, &env.runtime)?;

        instance.use_gas(&mut store, LOAD_MLDSA_PUBLIC_KEY_GAS_COST);

        instance
            .write_memory(&store, result_ptr as u64, &response)
            .map_err(|_e| RuntimeError::new("Error writing mldsa to memory"))?;

        Ok(())
    }
}
