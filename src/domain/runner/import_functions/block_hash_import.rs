use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

const COLD_BLOCK_ACCESS_GAS_COST: u64 = 26_000_000;
const WARM_BLOCK_ACCESS_GAS_COST: u64 = 1_000_000;

#[derive(Default)]
pub struct GetBlockHashImport;

impl GetBlockHashImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        block_number: u64,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new(
                "Cannot get block hash in start function",
            ));
        }
        
        let instance = &env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let result = env
            .block_hash_external
            .execute(block_number, &env.runtime)?;

        let block_access_cost = if result.is_block_warm {
            WARM_BLOCK_ACCESS_GAS_COST
        } else {
            COLD_BLOCK_ACCESS_GAS_COST
        };

        instance.use_gas(&mut store, block_access_cost);

        instance
            .write_memory(&store, result_ptr as u64, &result.block_hash)
            .map_err(|_e| RuntimeError::new("Error writing block hash to memory"))?;

        Ok(())
    }
}
