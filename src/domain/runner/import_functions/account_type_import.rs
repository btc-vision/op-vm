use crate::domain::runner::{
    CustomEnv, COLD_ADDRESS_ACCESS_GAS_COST, WARM_ADDRESS_ACCESS_GAS_COST,
};
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct GetAccountTypeImport;

#[allow(dead_code)]
impl GetAccountTypeImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        address_ptr: u32,
    ) -> Result<u32, RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new(
                "Cannot get account type in start function",
            ));
        }

        let instance = &env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let address_hash = instance
            .read_memory(&store, address_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading address hash from memory"))?;

        let result = env
            .account_type_external
            .execute(&address_hash, &env.runtime)?;

        let address_access_cost = if result.is_address_warm {
            WARM_ADDRESS_ACCESS_GAS_COST
        } else {
            COLD_ADDRESS_ACCESS_GAS_COST
        };

        instance.use_gas(&mut store, address_access_cost);

        Ok(result.account_type)
    }
}
