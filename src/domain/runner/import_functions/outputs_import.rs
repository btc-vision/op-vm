use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::{CustomEnv, OUTPUTS_COST};
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct OutputsImport;

impl OutputsImport {
    pub fn execute(mut context: FunctionEnvMut<CustomEnv>) -> Result<u32, RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, OUTPUTS_COST);

        let result = &env.outputs_external.execute(&env.runtime)?;
        let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
            .map_err(|e| RuntimeError::new(format!("Error writing buffer: {}", e)))?;

        Ok(value as u32)
    }
}
