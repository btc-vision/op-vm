use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

const STATIC_GAS_COST: u64 = 1_000_000;

#[derive(Default)]
pub struct AddressTypeImport;

impl AddressTypeImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        address_ptr: u32,
    ) -> Result<u32, RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();
        let instance = &env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;
        let address_hash = instance
            .read_memory(&store, address_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading address hash from memory"))?;

        instance.use_gas(&mut store, STATIC_GAS_COST);
        env.address_type_external
            .execute(&address_hash, &env.runtime)
    }
}
