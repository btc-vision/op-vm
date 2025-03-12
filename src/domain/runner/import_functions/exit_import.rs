use crate::domain::runner::{CustomEnv, ExitData, ExitResult};
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct ExitImport;

impl ExitImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        status: u32,
        data_ptr: u32,
        data_length: u32,
    ) -> Result<(), ExitResult> {
        let result = Self::get_exit_data(&mut context, status, data_ptr, data_length);
        Err(match result {
            Ok(data) => ExitResult::Ok(data),
            Err(e) => ExitResult::Err(e),
        })
    }

    fn get_exit_data(
        context: &mut FunctionEnvMut<CustomEnv>,
        status: u32,
        data_ptr: u32,
        data_length: u32,
    ) -> Result<ExitData, RuntimeError> {
        let (env, store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let data = instance
            .read_memory(&store, data_ptr as u64, data_length as u64)
            .map_err(|_e| RuntimeError::new("Error reading exit data from memory"))?;

        env.exit_data = ExitData::new(status, data.as_slice());

        Ok(ExitData::new(status, data.as_slice()))
    }
}
