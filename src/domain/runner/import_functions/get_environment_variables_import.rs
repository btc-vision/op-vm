use crate::domain::runner::import_functions::common::DataSliceWriter;
use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

const STATIC_GAS_COST: u64 = 200_000;

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

        if env.is_running_start_function {
            return Err(RuntimeError::new(
                "Cannot get environment variables in start function",
            ));
        }

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        env.charge_gas(&instance, &mut store, STATIC_GAS_COST)?;

        let environment_variables = &env
            .environment_variables
            .clone()
            .ok_or(RuntimeError::new("Environment not found"))?
            .serialize_for_contract();

        DataSliceWriter::write_data_and_padding_to_memory_with_limit(
            &mut store,
            &instance,
            environment_variables,
            offset,
            length,
            result_ptr,
            env.is_strict_memory_metering_enabled(),
        )
    }
}
