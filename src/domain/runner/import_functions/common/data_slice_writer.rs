use crate::domain::runner::InstanceWrapper;
use std::cmp::min;
use wasmer::{RuntimeError, StoreMut};

#[derive(Default)]
pub struct DataSliceWriter;

impl DataSliceWriter {
    pub fn write_data_and_padding_to_memory(
        store: &mut StoreMut,
        instance: &InstanceWrapper,
        result_data: &[u8],
        offset: u32,
        length: u32,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        Self::write_data_slice_to_memory(
            store,
            &instance,
            result_data,
            offset,
            length,
            result_ptr,
        )?;
        Self::write_data_padding_to_memory(store, &instance, result_data, offset, length, result_ptr)
    }

    fn write_data_slice_to_memory(
        store: &mut StoreMut,
        instance: &InstanceWrapper,
        result_data: &[u8],
        offset: u32,
        length: u32,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let result_data_sliced =
            Self::slice_in_bounds(result_data, offset as usize, length as usize)
                .ok_or(RuntimeError::new("Error slicing data"))?;

        if !result_data_sliced.is_empty() {
            instance
                .write_memory(&store, result_ptr as u64, result_data_sliced)
                .map_err(|_e| RuntimeError::new("Error writing data to memory"))?;
        }
        Ok(())
    }

    fn slice_in_bounds(data: &[u8], offset: usize, length: usize) -> Option<&[u8]> {
        let start = min(offset, data.len());
        let end = min(offset + length, data.len());
        data.get(start..end)
    }

    fn write_data_padding_to_memory(
        store: &mut StoreMut,
        instance: &InstanceWrapper,
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
                .map_err(|_e| RuntimeError::new("Error writing data to memory"))?;
        }
        Ok(())
    }
}
