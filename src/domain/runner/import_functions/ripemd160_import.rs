use crate::domain::runner::CustomEnv;
use ripemd::Ripemd160;
use sha2::Digest;
use wasmer::{FunctionEnvMut, RuntimeError};

const STATIC_GAS_COST: u64 = 300_000;
const GAS_COST_PER_WORD: u64 = 60_000;

#[derive(Default)]
pub struct Ripemd160Import;

impl Ripemd160Import {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        data_ptr: u32,
        data_length: u32,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let data = instance
            .read_memory(&store, data_ptr as u64, data_length as u64)
            .map_err(|_e| RuntimeError::new("Error reading data from memory"))?;

        let result = Self::ripemd160(&data)?;

        instance
            .write_memory(&store, result_ptr as u64, &result)
            .map_err(|_e| RuntimeError::new("Error writing result to memory"))?;

        instance.use_gas(
            &mut store,
            STATIC_GAS_COST + ((data.len() + 31) / 32) as u64 * GAS_COST_PER_WORD,
        );

        Ok(())
    }

    fn ripemd160(data: &[u8]) -> Result<Vec<u8>, RuntimeError> {
        let mut ripemd = Ripemd160::new();
        ripemd.update(data);

        let hash = ripemd.finalize();
        let hash_as_vec: Vec<u8> = hash.to_vec();

        Ok(hash_as_vec)
    }
}
