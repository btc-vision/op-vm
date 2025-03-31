use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

const STATIC_GAS_COST: u64 = 21_000_000;

#[derive(Default)]
pub struct GetBlockHashImport;

impl GetBlockHashImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        block_number: u64,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();
        let instance = &env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, STATIC_GAS_COST);
        
        let result = env.block_hash_external.execute(block_number, &env.runtime)?;

        instance
            .write_memory(&store, result_ptr as u64, &result)
            .map_err(|_e| RuntimeError::new("Error writing block hash to memory"))?;


        Ok(())
    }
}
