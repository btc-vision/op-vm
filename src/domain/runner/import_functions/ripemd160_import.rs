use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::CustomEnv;
use ripemd::Ripemd160;
use sha2::Digest;
use wasmer::{FunctionEnvMut, RuntimeError};

pub const STATIC_GAS_COST: u64 = 300_000;
pub const GAS_COST_PER_WORD: u64 = 60_000;

#[derive(Default)]
pub struct Ripemd160Import;

impl Ripemd160Import {
    pub fn execute(mut context: FunctionEnvMut<CustomEnv>, ptr: u32) -> Result<u32, RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let data = AssemblyScript::read_buffer(&store, &instance, ptr)
            .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

        let result = Self::ripemd160(&data)?;

        let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
            .map_err(|e| RuntimeError::new(format!("Error writing buffer: {}", e)))?;

        instance.use_gas(
            &mut store,
            STATIC_GAS_COST + ((data.len() + 31) / 32) as u64 * GAS_COST_PER_WORD,
        );

        Ok(value as u32)
    }

    fn ripemd160(data: &[u8]) -> Result<Vec<u8>, RuntimeError> {
        let mut ripemd = Ripemd160::new();
        ripemd.update(data);

        let hash = ripemd.finalize();
        let hash_as_vec: Vec<u8> = hash.to_vec();

        Ok(hash_as_vec)
    }
}
