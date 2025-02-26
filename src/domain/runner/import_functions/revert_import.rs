use crate::domain::runner::{CustomEnv, RevertData};
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct RevertImport;

impl RevertImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        data_ptr: u32,
        data_length: u32,
    ) -> Result<(), RuntimeError> {
        let (env, store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let data = instance
            .read_memory(&store, data_ptr as u64, data_length as u64)
            .map_err(|_e| RuntimeError::new("Error reading storage key from memory"))?;

        env.revert_data = Some(RevertData::new(data.as_slice()));

        Err(RuntimeError::new("Execution reverted"))
    }
}
