use anyhow::anyhow;
use napi::Error;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use wasmer::{Instance, Memory, MemoryAccessError, MemoryView, RuntimeError, StoreMut, Value};

use crate::domain::contract::abort_data::AbortData;
use crate::interfaces::ThreadSafeJsImportResponse;

pub struct CustomEnv {
    pub memory: Option<Memory>,
    pub abort_data: Option<AbortData>,
    pub load_function: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    pub instance: Option<Instance>,
}

impl CustomEnv {
    pub fn new(load_function: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>) -> anyhow::Result<Self> {
        Ok(Self {
            memory: None,
            abort_data: None,
            load_function,
            instance: None,
        })
    }

    pub fn __pin(
        &self,
        instance: &Instance,
        store: &mut StoreMut,
        pointer: i32,
    ) -> anyhow::Result<Box<[Value]>> {
        self.call(instance, store, "__pin", &[Value::I32(pointer)])
    }

    pub fn __unpin(
        &self,
        instance: &Instance,
        store: &mut StoreMut,
        pointer: i32,
    ) -> anyhow::Result<Box<[Value]>> {
        self.call(instance, store, "__unpin", &[Value::I32(pointer)])
    }

    pub fn __new(&self,
                 instance: &Instance,
                 store: &mut StoreMut, size: i32, id: i32) -> anyhow::Result<i32> {
        let params = &[Value::I32(size), Value::I32(id)];
        let result = self.call(instance, store, "__new", params)?;

        let pointer = result
            .get(0)
            .ok_or(anyhow!("can't get pointer"))?
            .i32()
            .ok_or(anyhow!("can't get pointer"))?;

        return Ok(pointer);
    }

    pub fn write_buffer(
        &self,
        instance: &Instance,
        store: &mut StoreMut,
        value: &[u8],
        id: i32,
        align: u32,
    ) -> Result<i64, Error> {
        // Calculate the length and create a new buffer
        let length = value.len();
        let buffer_size = length << align;
        let buffer = self.__new(instance, store, buffer_size as i32, 1);
        if buffer.is_err() {
            return Err(Error::from_reason(format!(
                "Failed to get buffer from __new: {:?}",
                buffer.unwrap_err()
            )));
        }

        let buffer_value = buffer.unwrap();

        // Pin the buffer
        let pinned_buffer = self.__pin(instance, store, buffer_value);
        if pinned_buffer.is_err() {
            return Err(Error::from_reason(format!(
                "Failed to pin buffer: {:?}",
                pinned_buffer.unwrap_err()
            )));
        }

        let pin_value: Value = pinned_buffer.unwrap()[0].clone();
        let pinned_buffer_value = pin_value.unwrap_i32() as u32;

        // Create the header
        let header = self.__new(instance, store, 12, id);
        if header.is_err() {
            return Err(Error::from_reason(format!(
                "Failed to get header from __new: {:?}",
                header.unwrap_err()
            )));
        }

        let header_value = header.unwrap();

        {
            let view = self.memory.clone().unwrap().view(store);

            // Set the header values
            self.set_u32(&view, header_value, pinned_buffer_value).unwrap();
            self.set_u32(&view, header_value + 4, pinned_buffer_value).unwrap();
            self.set_u32(&view, header_value + 8, buffer_size as u32).unwrap();

            // Write the buffer value to the contract's memory
            self.write_memory(&view, pinned_buffer_value as u64, &value).unwrap();
        }

        // Unpin the buffer
        self.__unpin(instance, store, pinned_buffer_value as i32).unwrap();

        return Ok(header_value as i64);
    }

    pub fn set_u32(&self,
                   view: &MemoryView, offset: i32, value: u32) -> Result<(), MemoryAccessError> {
        self.write_memory(view, offset as u64, &value.to_le_bytes())
    }

    pub fn write_memory(&self, view: &MemoryView, offset: u64, data: &[u8]) -> Result<(), MemoryAccessError> {
        view.write(offset, data)
    }

    pub fn read_memory(view: &MemoryView, offset: u64, length: u64) -> Result<Vec<u8>, RuntimeError> {
        let mut buffer: Vec<u8> = vec![0; length as usize];
        view.read(offset, &mut buffer).unwrap();

        Ok(buffer)
    }

    pub fn call(&self, instance: &Instance, store: &mut StoreMut, function: &str, params: &[Value]) -> anyhow::Result<Box<[Value]>> {
        let export = instance.exports.get_function(function)?; //Self::get_function(&self.instance, function)?;
        let result = export.call(store, params)?;
        Ok(result)
    }

    pub fn read_pointer(&self, memory_view: &MemoryView, offset: u64, length: u64) -> Result<Vec<u8>, RuntimeError> {
        let mut buffer: Vec<u8> = vec![0; length as usize];
        for i in 0..length {
            let byte = memory_view.read_u8(offset + i);

            // check for error
            if byte.is_err() {
                return Err(RuntimeError::new("Out of bounds memory access"));
            }

            buffer[i as usize] = byte.unwrap();
        }

        Ok(buffer)
    }

    pub fn bytes_to_u32_le(&self, bytes: Vec<u8>) -> u32 {
        let mut result = 0;
        for i in 0..4 {
            result |= (bytes[i] as u32) << (i * 8);
        }
        result
    }

    pub fn read_buffer(&self, memory_view: &MemoryView, offset: i32) -> Result<Vec<u8>, Error> {
        let pointer = self.read_pointer(memory_view, (offset + 4) as u64, 4);
        if pointer.is_err() {
            return Err(Error::from_reason("Failed to read length"));
        }

        let pointer_buffer = pointer.unwrap();
        let pointer = self.bytes_to_u32_le(pointer_buffer);

        //println!("Pointer: {}", pointer);

        let length = self.read_pointer(memory_view, (offset + 8) as u64, 4);
        if length.is_err() {
            return Err(Error::from_reason("Failed to read length"));
        }

        let length_buffer = length.unwrap();
        let length = self.bytes_to_u32_le(length_buffer);

        //println!("Length: {}", length);

        let result = self.read_pointer(memory_view, pointer as u64 + 4, length as u64);
        if result.is_err() {
            return Err(Error::from_reason(format!("{:?}", result.unwrap_err())));
        }

        Ok(result.unwrap())
    }
}
