use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::{CustomEnv, INPUTS_COST};
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct InputsImport;

impl InputsImport {
    pub fn execute(mut context: FunctionEnvMut<CustomEnv>) -> Result<u32, RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, INPUTS_COST);

        let result = &env.inputs_external.execute(&env.runtime)?;
        let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
            .map_err(|e| RuntimeError::new(format!("Error writing buffer: {}", e)))?;

        Ok(value as u32)
    }
}
