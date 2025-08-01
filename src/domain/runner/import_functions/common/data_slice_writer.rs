use std::cmp::min;
use wasmer::{AsStoreRef, RuntimeError, StoreMut};

pub trait MemoryWriter {
    type Error: std::error::Error;

    fn write_memory(
        &self,
        store: &impl AsStoreRef,
        offset: u64,
        data: &[u8],
    ) -> Result<(), Self::Error>;
}

#[derive(Default)]
pub struct DataSliceWriter;

impl DataSliceWriter {
    pub fn write_data_and_padding_to_memory<W: MemoryWriter>(
        store: &mut StoreMut,
        instance: &W,
        result_data: &[u8],
        offset: u32,
        length: u32,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        let data_written = Self::write_data_slice_to_memory(
            store,
            instance,
            result_data,
            offset,
            length,
            result_ptr,
        )?;

        Self::write_data_padding_to_memory(store, instance, data_written, length, result_ptr)
    }

    fn write_data_slice_to_memory<W: MemoryWriter>(
        store: &mut StoreMut,
        instance: &W,
        result_data: &[u8],
        offset: u32,
        length: u32,
        result_ptr: u32,
    ) -> Result<usize, RuntimeError> {
        let result_data_sliced =
            Self::slice_in_bounds(result_data, offset as usize, length as usize)
                .ok_or(RuntimeError::new("Error slicing data"))?;

        let data_written = result_data_sliced.len();

        if !result_data_sliced.is_empty() {
            instance
                .write_memory(store, result_ptr as u64, result_data_sliced)
                .map_err(|_e| RuntimeError::new("Error writing data to memory"))?;
        }

        Ok(data_written)
    }

    fn slice_in_bounds(data: &[u8], offset: usize, length: usize) -> Option<&[u8]> {
        let start = min(offset, data.len());
        let end = min(offset + length, data.len());
        data.get(start..end)
    }

    fn write_data_padding_to_memory<W: MemoryWriter>(
        store: &mut StoreMut,
        instance: &W,
        data_written: usize,
        length: u32,
        result_ptr: u32,
    ) -> Result<(), RuntimeError> {
        if data_written < length as usize {
            let zero_bytes_length = length as usize - data_written;
            let zero_bytes_vec = vec![0; zero_bytes_length];

            let ptr = result_ptr as u64 + data_written as u64;

            instance
                .write_memory(store, ptr, &zero_bytes_vec)
                .map_err(|_e| RuntimeError::new("Error writing data to memory"))?;
        }
        Ok(())
    }
}
