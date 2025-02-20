use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::{exported_import_functions, CustomEnv};
use wasmer::{FunctionEnvMut, RuntimeError};

pub const STATIC_GAS_COST: u64 = 1_000_000;
pub const GAS_COST_PER_WORD: u64 = 120_000;

#[derive(Default)]
pub struct ValidateBitcoinAddressImport;

impl ValidateBitcoinAddressImport {
    // TODO: Add support for other blockchains
    pub fn execute(mut context: FunctionEnvMut<CustomEnv>, ptr: u32) -> Result<u32, RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let data = AssemblyScript::read_buffer(&store, &instance, ptr)
            .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;
        let data_len = data.len() as u64;

        let string_data = String::from_utf8(data)
            .map_err(|e| RuntimeError::new(format!("Error converting to string: {}", e)))?;
        let result =
            exported_import_functions::validate_bitcoin_address(&string_data, &env.network)
                .map_err(|e| RuntimeError::new(e))?;

        let result_vec_buffer = vec![result as u8];

        let value = AssemblyScript::write_buffer(&mut store, &instance, &result_vec_buffer, 13, 0)
            .map_err(|e| RuntimeError::new(format!("Error writing buffer: {}", e)))?;

        instance.use_gas(
            &mut store,
            STATIC_GAS_COST
                + ((data_len + 31) / 32) * GAS_COST_PER_WORD,
        );

        Ok(value as u32)
    }
}
