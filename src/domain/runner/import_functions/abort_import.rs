use crate::domain::runner::{AbortData, CustomEnv};
use wasmer::{FunctionEnvMut, RuntimeError};

#[derive(Default)]
pub struct AbortImport;

impl AbortImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        message: u32,
        file_name: u32,
        line: u32,
        column: u32,
    ) -> Result<(), RuntimeError> {
        let data = context.data_mut();
        data.abort_data = Some(AbortData {
            message,
            file_name,
            line,
            column,
        });

        Err(RuntimeError::new("Execution aborted"))
    }
}
