use crate::domain::runner::{CustomEnv, InstanceWrapper};
use std::cmp::min;
use wasmer::{FunctionEnvMut, RuntimeError, StoreMut};

pub const STATIC_GAS_COST: u64 = 30_000;

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

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, STATIC_GAS_COST);

        let result_data = env.last_call_result.data.as_slice();

        Self::write_call_result_data_slice_to_memory(
            &mut store,
            &instance,
            result_data,
            offset,
            length,
            result_ptr,
        )?;

        Self::write_call_result_data_padding_to_memory(
            &mut store,
            instance,
            result_data,
            offset,
            length,
            result_ptr,
        )?;

        Ok(())
    }

    fn write_call_result_data_slice_to_memory(
        store: &mut StoreMut,
        instance: &InstanceWrapper,
        result_data: &[u8],
        offset: u32,
        length: u32,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let result_data_sliced =
            Self::slice_in_bounds(result_data, offset as usize, length as usize)
                .ok_or(RuntimeError::new("Error slicing call result"))?;

        if !result_data_sliced.is_empty() {
            instance
                .write_memory(&store, result_ptr as u64, result_data_sliced)
                .map_err(|_e| RuntimeError::new("Error writing call result to memory"))?;
        }
        Ok(())
    }

    fn slice_in_bounds(data: &[u8], offset: usize, length: usize) -> Option<&[u8]> {
        let start = min(offset, data.len());
        let end = min(offset + length, data.len());
        data.get(start..end)
    }

    fn write_call_result_data_padding_to_memory(
        store: &mut StoreMut,
        instance: InstanceWrapper,
        result_data: &[u8],
        offset: u32,
        length: u32,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        if result_data.len() < (offset + length) as usize {
            let zero_bytes_length = (offset + length) as usize - result_data.len();
            let zero_bytes_vec = vec![0; zero_bytes_length];
            let ptr = result_ptr as u64 + result_data.len() as u64;
            instance
                .write_memory(&store, ptr, &zero_bytes_vec)
                .map_err(|_e| RuntimeError::new("Error writing call result to memory"))?;
        }
        Ok(())
    }
}
