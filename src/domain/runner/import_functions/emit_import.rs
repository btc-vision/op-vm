use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::{CustomEnv, EMIT_COST};
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct EmitImport;

impl EmitImport {
    pub fn execute(mut context: FunctionEnvMut<CustomEnv>, ptr: u32) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();
        let instance = &env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Memory not found"))?;

        instance.use_gas(&mut store, EMIT_COST);

        let data = AssemblyScript::read_buffer(&store, &instance, ptr)
            .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

        env.emit_external.execute(&data, &env.runtime)
    }
}
