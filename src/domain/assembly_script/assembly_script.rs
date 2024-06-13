use anyhow::anyhow;
use napi::Error;
use wasmer::{RuntimeError, Value};

use crate::domain::contract::Contract;

pub struct AssemblyScript;

impl AssemblyScript {
    pub fn init(contract: &mut Contract, address: &str, deployer: &str) {
        let contract_address: i32 =
            AssemblyScript::lower_string(contract, &address).unwrap() as i32;
        let deployer_address: i32 =
            AssemblyScript::lower_string(contract, &deployer).unwrap() as i32;

        contract
            .call(
                "INIT",
                &[Value::I32(contract_address), Value::I32(deployer_address)],
            )
            .unwrap();
    }

    pub fn __new(contract: &mut Contract, size: i32, id: i32) -> anyhow::Result<i32> {
        let params = &[Value::I32(size), Value::I32(id)];
        let result = contract.call("__new", params)?;

        let pointer = result
            .get(0)
            .ok_or(anyhow!("can't get pointer"))?
            .i32()
            .ok_or(anyhow!("can't get pointer"))?;

        return Ok(pointer);
    }

    pub fn __pin(contract: &mut Contract, pointer: i32) -> Result<Box<[Value]>, RuntimeError> {
        contract.call("__pin", &[Value::I32(pointer)])
    }

    pub fn __unpin(contract: &mut Contract, pointer: i32) -> Result<Box<[Value]>, RuntimeError> {
        contract.call("__unpin", &[Value::I32(pointer)])
    }

    pub fn lower_string(contract: &mut Contract, value: &str) -> anyhow::Result<u32> {
        let length: i32 = value.len().try_into()?;

        let result = contract.call("__new", &[Value::I32(length << 1), Value::I32(2)])?;

        let pointer = result
            .get(0)
            .ok_or(anyhow!("can't get new string pointer"))?
            .i32()
            .ok_or(anyhow!("can't get new string pointer"))?;

        let utf16: Vec<u16> = value.encode_utf16().collect();
        let utf16_to_u8: &[u8] = bytemuck::try_cast_slice(&utf16.as_slice()).expect("qaq");

        contract.write_memory(pointer as u64, utf16_to_u8).unwrap();
        contract.call("__pin", &[Value::I32(pointer)])?;

        Ok(pointer as u32)
    }

    pub fn write_buffer(
        contract: &mut Contract,
        value: Vec<u8>,
        id: i32,
        align: u32,
    ) -> Result<i64, Error> {
        // Calculate the length and create a new buffer
        let length = value.len();
        let buffer_size = length << align;
        let buffer = Self::__new(contract, buffer_size as i32, 1);
        if buffer.is_err() {
            return Err(Error::from_reason(format!(
                "Failed to get buffer from __new: {:?}",
                buffer.unwrap_err()
            )));
        }

        let buffer_value = buffer.unwrap();

        // Pin the buffer
        let pinned_buffer = Self::__pin(contract, buffer_value);
        if pinned_buffer.is_err() {
            return Err(Error::from_reason(format!(
                "Failed to pin buffer: {:?}",
                pinned_buffer.unwrap_err()
            )));
        }

        let pin_value: Value = pinned_buffer.unwrap()[0].clone();
        let pinned_buffer_value = pin_value.unwrap_i32() as u32;

        // Create the header
        let header = Self::__new(contract, 12, id);
        if header.is_err() {
            return Err(Error::from_reason(format!(
                "Failed to get header from __new: {:?}",
                header.unwrap_err()
            )));
        }

        let header_value = header.unwrap();

        // Set the header values
        contract.set_u32(header_value, pinned_buffer_value).unwrap();
        contract
            .set_u32(header_value + 4, pinned_buffer_value)
            .unwrap();
        contract
            .set_u32(header_value + 8, buffer_size as u32)
            .unwrap();

        // Write the buffer value to the contract's memory
        contract
            .write_pointer(pinned_buffer_value as u64, value)
            .unwrap();

        // Unpin the buffer
        Self::__unpin(contract, pinned_buffer_value as i32).unwrap();

        return Ok(header_value as i64);
    }

    pub fn lift_typed_array(contract: &Contract, offset: i32) -> Result<Vec<u8>, Error> {
        let pointer = contract.read_pointer((offset + 4) as u64, 4);
        if pointer.is_err() {
            return Err(Error::from_reason("Failed to read length"));
        }

        let pointer_buffer = pointer.unwrap();
        let pointer = Self::bytes_to_u32_le(pointer_buffer);

        println!("Pointer: {}", pointer);

        let length = contract.read_pointer((offset + 8) as u64, 4);
        if length.is_err() {
            return Err(Error::from_reason("Failed to read length"));
        }

        let length_buffer = length.unwrap();
        let length = Self::bytes_to_u32_le(length_buffer);

        println!("Length: {}", length);
        let result = contract.read_pointer(pointer as u64 + 4, length as u64);
        if result.is_err() {
            return Err(Error::from_reason(format!("{:?}", result.unwrap_err())));
        }

        Ok(result.unwrap())
    }

    fn bytes_to_u32_le(bytes: Vec<u8>) -> u32 {
        let mut result = 0;
        for i in 0..4 {
            result |= (bytes[i] as u32) << (i * 8);
        }
        result
    }
}
