use crate::domain::runner::CustomEnv;
use crate::domain::vm::MAX_ACCUM;
use sha2::{Digest, Sha256};
use wasmer::{FunctionEnvMut, RuntimeError};

const STATIC_GAS_COST: u64 = 1_000_000;
const PER_BLOCK: u64 = 34_000;
const MAX: u64 = MAX_ACCUM;

#[derive(Default)]
pub struct Sha256Import;

impl Sha256Import {
    #[inline]
    fn sha256_gas(len: u64) -> u64 {
        let blocks = 1 + (len.saturating_sub(1) >> 6);
        STATIC_GAS_COST
            .saturating_add(PER_BLOCK.saturating_mul(blocks.saturating_sub(1)))
            .min(MAX)
    }

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

        instance.use_gas(&mut store, STATIC_GAS_COST);

        let data = instance
            .read_memory(&store, data_ptr as u64, data_length as u64)
            .map_err(|_e| RuntimeError::new("Error reading data from memory"))?;

        instance.use_gas(&mut store, Self::sha256_gas(data.len() as u64));

        let result = Self::sha256(&data)?;

        instance
            .write_memory(&store, result_ptr as u64, &result)
            .map_err(|_e| RuntimeError::new("Error writing result to memory"))?;

        Ok(())
    }

    fn sha256(data: &[u8]) -> Result<Vec<u8>, RuntimeError> {
        let hash = Sha256::digest(data);
        let hash_as_vec: Vec<u8> = hash.to_vec();

        Ok(hash_as_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_hashes_number_correctly() {
        let data_to_hash = vec![9];
        let expected_hash =
            hex::decode("2b4c342f5433ebe591a1da77e013d1b72475562d48578dca8b84bac6651c3cb9")
                .unwrap();

        let result = Sha256Import::sha256(&data_to_hash).unwrap();

        assert_eq!(result, expected_hash);
    }

    #[test]
    fn sha256_hashes_hex_data_correctly() {
        let data_to_hash = hex::decode("e3b0c44298fc1c149afbf4c8").unwrap().to_vec();
        let expected_hash =
            hex::decode("10dac508c2a7d7f0f3474c6ecc23f2a4d9ddbabec1009c4810f2ff677f4c1a83")
                .unwrap();

        let result = Sha256Import::sha256(&data_to_hash).unwrap();

        assert_eq!(result, expected_hash);
    }
}
