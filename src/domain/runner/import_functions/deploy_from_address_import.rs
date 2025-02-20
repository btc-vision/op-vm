use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::CustomEnv;
use crate::interfaces::ExternalFunction;
use wasmer::{FunctionEnvMut, RuntimeError};

pub const STATIC_GAS_COST: u64 = 2_500_000_000;

#[derive(Default)]
pub struct DeployFromAddressImport;

impl DeployFromAddressImport {
    pub fn execute(mut context: FunctionEnvMut<CustomEnv>, ptr: u32) -> Result<u32, RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();
        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let data = AssemblyScript::read_buffer(&mut store, &instance, ptr)
            .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

        instance.use_gas(&mut store, STATIC_GAS_COST);
        let result = &env
            .deploy_from_address_external
            .execute(&data, &env.runtime)?;
        let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
            .map_err(|e| RuntimeError::new(format!("Error writing buffer: {}", e)))?;

        Ok(value as u32)
    }
}
