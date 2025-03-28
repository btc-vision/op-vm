use crate::domain::runner::import_functions::common::DataSliceWriter;
use crate::domain::runner::CustomEnv;
use wasmer::{FunctionEnvMut, RuntimeError};

const STATIC_GAS_COST: u64 = 30_000;
const GAS_COST_PER_BYTE: u64 = 1_000;

#[derive(Default)]
pub struct GetCallResultImport;

impl GetCallResultImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        offset: u32,
        length: u32,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new(
                "Cannot get call result in start function",
            ));
        }

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let result_data = env.last_call_result.data.as_slice();

        instance.use_gas(
            &mut store,
            STATIC_GAS_COST + result_data.len() as u64 * GAS_COST_PER_BYTE,
        );

        DataSliceWriter::write_data_and_padding_to_memory(
            &mut store,
            &instance,
            result_data,
            offset,
            length,
            result_ptr,
        )
    }
}
