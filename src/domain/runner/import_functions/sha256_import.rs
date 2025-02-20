use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::{CustomEnv, SHA256_STATIC_COST, SHA256_WORD_COST};
use sha2::{Digest, Sha256};
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct Sha256Import;

impl Sha256Import {
    pub fn execute(mut context: FunctionEnvMut<CustomEnv>, ptr: u32) -> Result<u32, RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let data = AssemblyScript::read_buffer(&store, &instance, ptr)
            .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

        let result = Self::sha256(&data)?;

        let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
            .map_err(|e| RuntimeError::new(format!("Error writing buffer: {}", e)))?;

        instance.use_gas(
            &mut store,
            SHA256_STATIC_COST + ((data.len() + 31) / 32) as u64 * SHA256_WORD_COST,
        );

        Ok(value as u32)
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
