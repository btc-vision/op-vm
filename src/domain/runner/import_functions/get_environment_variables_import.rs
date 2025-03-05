use crate::domain::runner::import_functions::common::DataSliceWriter;
use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

pub const STATIC_GAS_COST: u64 = 100_000;

#[derive(Default)]
pub struct GetEnvironmentVariablesImport;

impl GetEnvironmentVariablesImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        offset: u32,
        length: u32,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let environment_variables = &env
            .environment_variables
            .clone()
            .ok_or(RuntimeError::new("Environment not found"))?
            .serialize_for_contract();

        instance.use_gas(&mut store, STATIC_GAS_COST);

        DataSliceWriter::write_data_and_padding_to_memory(
            &mut store,
            &instance,
            environment_variables,
            offset,
            length,
            result_ptr,
        )
    }
}
