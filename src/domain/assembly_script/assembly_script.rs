use anyhow::anyhow;
use napi::Error;
use wasmer::{AsStoreMut, AsStoreRef, RuntimeError, Value};

use crate::domain::runner::{ExtendedMemoryAccessError, InstanceWrapper, MAX_LENGTH_BUFFER_EXTERN};

pub struct AssemblyScript;

impl AssemblyScript {
    pub fn __new(
        store: &mut impl AsStoreMut,
        instance: &InstanceWrapper,
        size: i32,
        id: i32,
    ) -> anyhow::Result<i32> {
        let params = &[Value::I32(size), Value::I32(id)];
        let result = instance.call(store, "__new", params)?;

        let pointer = result
            .get(0)
            .ok_or(anyhow!("can't get pointer"))?
            .i32()
            .ok_or(anyhow!("can't get pointer"))?;

        Ok(pointer)
    }

    pub fn __pin(
        store: &mut impl AsStoreMut,
        instance: &InstanceWrapper,
        pointer: i32,
    ) -> anyhow::Result<Box<[Value]>> {
        instance.call(store, "__pin", &[Value::I32(pointer)])
    }

    pub fn __unpin(
        store: &mut impl AsStoreMut,
        instance: &InstanceWrapper,
        pointer: i32,
    ) -> anyhow::Result<Box<[Value]>> {
        instance.call(store, "__unpin", &[Value::I32(pointer)])
    }

    pub fn write_buffer(
        store: &mut impl AsStoreMut,
        instance: &InstanceWrapper,
        value: &[u8],
        id: i32,
        align: u32,
    ) -> Result<i64, Error> {
        // Calculate the length and create a new buffer
        let length = value.len();
        let buffer_size = length << align;
        let buffer = Self::__new(store, instance, buffer_size as i32, 1);
        if buffer.is_err() {
            return Err(Error::from_reason(format!(
                "Failed to get buffer from __new: {:?}",
                buffer.unwrap_err()
            )));
        }

        let buffer_value = buffer.unwrap();

        // Pin the buffer
        let pinned_buffer = Self::__pin(store, instance, buffer_value);
        if pinned_buffer.is_err() {
            return Err(Error::from_reason(format!(
                "Failed to pin buffer: {:?}",
                pinned_buffer.unwrap_err()
            )));
        }

        let pin_value: Value = pinned_buffer.unwrap()[0].clone();
        let pinned_buffer_value = pin_value.unwrap_i32() as u32;

        // Create the header
        let header = Self::__new(store, instance, 12, id);
        if header.is_err() {
            return Err(Error::from_reason(format!(
                "Failed to get header from __new: {:?}",
                header.unwrap_err()
            )));
        }

        let header_value = header.map_err(|e| {
            Error::from_reason(format!("Failed to get header from __new: {:?}", e))
        })?;

        // Set the header values
        Self::set_u32(store, instance, header_value, pinned_buffer_value).map_err(|e| {
            Error::from_reason(format!("Failed to set header value: {:?}", e))
        })?;
        Self::set_u32(store, instance, header_value + 4, pinned_buffer_value).map_err(|e| {
            Error::from_reason(format!("Failed to set header value: {:?}", e))
        })?;
        Self::set_u32(store, instance, header_value + 8, buffer_size as u32).map_err(|e| {
            Error::from_reason(format!("Failed to set header value: {:?}", e))
        })?;

        // Write the buffer value to the contract's memory
        instance
            .write_memory(store, pinned_buffer_value as u64, &value).map_err(|e| {
            Error::from_reason(format!("Failed to write buffer to memory: {:?}", e))
        })?;

        // Unpin the buffer
        Self::__unpin(store, instance, pinned_buffer_value as i32).map_err(|e| {
            Error::from_reason(format!("Failed to unpin buffer: {:?}", e))
        })?;

        Ok(header_value as i64)
    }

    pub fn set_u32(
        store: &(impl AsStoreRef + ?Sized),
        instance: &InstanceWrapper,
        offset: i32,
        value: u32,
    ) -> Result<(), ExtendedMemoryAccessError> {
        instance.write_memory(store, offset as u64, &value.to_le_bytes())
    }

    pub fn read_pointer(
        store: &(impl AsStoreRef + ?Sized),
        instance: &InstanceWrapper,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, RuntimeError> {
        let mut buffer: Vec<u8> = vec![0; length as usize];
        for i in 0..length {
            let byte = instance.read_memory_u8(store, offset + i);

            // check for error
            if byte.is_err() {
                return Err(RuntimeError::new(format!(
                    "Failed to read byte at offset {} length {}. Error: {:?}",
                    offset + i,
                    length,
                    byte.unwrap_err()
                )));
            }

            buffer[i as usize] = byte.map_err(|e| RuntimeError::new(e.to_string()))?;
        }

        Ok(buffer)
    }

    pub fn read_buffer(
        store: &(impl AsStoreRef + ?Sized),
        instance: &InstanceWrapper,
        offset: u32,
    ) -> Result<Vec<u8>, Error> {
        let pointer = Self::read_pointer(store, instance, (offset + 4) as u64, 8);
        let pointer_buffer = pointer.map_err(|e| Error::from_reason(e.to_string()))?;

        let data_offset = Self::bytes_to_u32_le(pointer_buffer.clone(), 0);
        let length = Self::bytes_to_u32_le(pointer_buffer, 4);

        // Make sure the length is less than x MB
        if length > MAX_LENGTH_BUFFER_EXTERN {
            return Err(Error::from_reason(format!(
                "External buffer too large ({} > {})",
                length,
                MAX_LENGTH_BUFFER_EXTERN
            )));
        }

        let result = Self::read_pointer(store, instance, data_offset as u64, length as u64)
            .map_err(|e| Error::from_reason(e.to_string()))?;

        Ok(result)
    }

    fn bytes_to_u32_le(bytes: Vec<u8>, offset: u32) -> u32 {
        let mut result = 0;
        for i in 0..4 {
            result |= (bytes[i + offset as usize] as u32) << (i * 8);
        }
        result
    }
}
