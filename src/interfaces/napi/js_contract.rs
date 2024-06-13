use napi::{Env, Error, JsNumber, JsUnknown, Result};
use napi::bindgen_prelude::{Array, BigInt, Buffer, Undefined};
use wasmer::Value;
use wasmer_types::RawValue;

use crate::domain::Contract;

#[napi(js_name = "Contract")]
pub struct JsContract {
    contract: Contract,
}

impl JsContract {
    fn value_to_js(env: &Env, value: &Value) -> Result<JsUnknown> {
        match value {
            Value::I32(v) => {
                let js_value = env.create_int32(*v).unwrap();
                let unknown = js_value.into_unknown();

                Ok(unknown)
            }
            Value::I64(v) => {
                let js_value = env.create_int64(*v).unwrap();
                let unknown = js_value.into_unknown();

                Ok(unknown)
            }

            Value::F32(v) => {
                let js_value = env.create_double(*v as f64).unwrap();
                let unknown = js_value.into_unknown();

                Ok(unknown)
            }

            Value::F64(v) => {
                let js_value = env.create_double(*v).unwrap();
                let unknown = js_value.into_unknown();

                Ok(unknown)
            }

            Value::V128(v) => {
                let js_value = env.create_bigint_from_u128(*v).unwrap();
                let unknown = js_value.into_unknown().unwrap();

                Ok(unknown)
            }

            _ => Err(Error::from_reason("Unsupported value type")),
        }
    }

    fn box_values_to_js_array(env: &Env, values: Box<[Value]>) -> Result<Array> {
        let vals: Vec<Value> = values.clone().into_vec();
        let mut js_array = env.create_array(vals.len() as u32).unwrap();

        for value in values.iter() {
            let js_value = JsContract::value_to_js(env, value).unwrap();
            let _ = js_array.insert(js_value);
        }

        Ok(js_array)
    }

    fn bytes_to_u32_le(bytes: Vec<u8>) -> u32 {
        let mut result = 0;
        for i in 0..4 {
            result |= (bytes[i] as u32) << (i * 8);
        }

        result
    }
}

#[napi] //noinspection RsCompileErrorMacro
impl JsContract {
    #[napi(constructor)]
    pub fn new(bytecode: Buffer, address: String, deployer: String) -> Self {
        let bytecode_vec = bytecode.to_vec();
        let contract = Contract::new(&bytecode_vec, &address, &deployer);
        Self { contract }
    }

    #[napi]
    pub fn init(&mut self) {
        self.contract.init();
    }

    #[napi]
    pub fn read_memory(&mut self, offset: BigInt, length: BigInt) -> Result<Buffer> {
        let offset = offset.get_u64().1;
        let length = length.get_u64().1;

        let result = self.contract.read_memory(offset, length).unwrap();

        return Ok(Buffer::from(result));
    }

    #[napi]
    pub fn write_memory(&self, offset: BigInt, data: Buffer) -> Result<Undefined> {
        let data: Vec<u8> = data.into();
        let offset = offset.get_u64().1;

        let result = self.contract.write_memory(offset, &data);
        if result.is_err() {
            return Err(Error::from_reason(format!("{:?}", result.unwrap_err())));
        }

        return Ok(());
    }

    #[napi]
    pub fn lower_string(&mut self, value: String) -> Result<u32> {
        let result = self.contract
            .lower_string(&value)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

        return Ok(result);
    }

    #[napi]
    pub fn write_buffer(
        &mut self,
        value: Buffer,
        id: i32,
        align: u32,
    ) -> Result<i64> {
        // Convert Buffer to Vec<u8>
        let value = value.to_vec();

        let contract = &mut self.contract;

        // Calculate the length and create a new buffer
        let length = value.len();
        let buffer_size = length << align;
        let buffer = contract.__new(buffer_size as i32, 1);
        if buffer.is_err() {
            return Err(Error::from_reason(format!(
                "Failed to get buffer from __new: {:?}",
                buffer.unwrap_err()
            )));
        }

        let buffer_value = buffer.unwrap();

        // Pin the buffer
        let pinned_buffer = contract.__pin(buffer_value);
        if pinned_buffer.is_err() {
            return Err(Error::from_reason(format!(
                "Failed to pin buffer: {:?}",
                pinned_buffer.unwrap_err()
            )));
        }

        let pin_value: Value = pinned_buffer.unwrap()[0].clone();
        let pinned_buffer_value = pin_value.unwrap_i32() as u32;

        // Create the header
        let header = contract.__new(12, id);
        if header.is_err() {
            return Err(Error::from_reason(format!(
                "Failed to get header from __new: {:?}",
                header.unwrap_err()
            )));
        }

        let header_value = header.unwrap();

        // Set the header values
        contract.set_u32(header_value, pinned_buffer_value).unwrap();
        contract.set_u32(header_value + 4, pinned_buffer_value).unwrap();
        contract.set_u32(header_value + 8, buffer_size as u32).unwrap();

        // Write the buffer value to the contract's memory
        contract.write_pointer(pinned_buffer_value as u64, value).unwrap();

        // Unpin the buffer
        contract.__unpin(pinned_buffer_value as i32).unwrap();

        return Ok(header_value as i64);
    }

    #[napi]
    pub fn lift_typed_array(&self, offset: i32) -> Result<Buffer> {
        let contract = &self.contract;

        let pointer = contract.read_pointer((offset + 4) as u64, 4);
        if pointer.is_err() {
            return Err(Error::from_reason("Failed to read length"));
        }

        let pointer_buffer = pointer.unwrap();
        let pointer = JsContract::bytes_to_u32_le(pointer_buffer);

        println!("Pointer: {}", pointer);

        let length = contract.read_pointer((offset + 8) as u64, 4);
        if length.is_err() {
            return Err(Error::from_reason("Failed to read length"));
        }

        let length_buffer = length.unwrap();
        let length = JsContract::bytes_to_u32_le(length_buffer);

        println!("Length: {}", length);
        let result = contract.read_pointer(pointer as u64 + 4, length as u64);
        if result.is_err() {
            return Err(Error::from_reason(format!("{:?}", result.unwrap_err())));
        }

        let buffer = result.unwrap();

        Ok(Buffer::from(buffer))
    }

    #[napi]
    pub fn call(&mut self, env: Env, func_name: String, params: Vec<JsNumber>) -> Result<Array> {
        let mut wasm_params = Vec::new();

        for param in params {
            let param_value = param.get_int32();
            if param_value.is_err() {
                return Err(Error::from_reason(format!(
                    "{:?}",
                    param_value.unwrap_err()
                )));
            }

            let value = param_value.unwrap();
            wasm_params.push(Value::I32(value));
        }

        let result = self.contract
            .call(&func_name, &wasm_params)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
        let js_array = JsContract::box_values_to_js_array(&env, result)?;

        Ok(js_array)
    }

    #[napi]
    pub fn call_test_1(&mut self, func_name: String) -> i32 {
        let response = self.contract.call(&func_name, &[]);
        let boxed = response.unwrap();
        let value: Value = boxed[0].clone();
        value.unwrap_i32()
    }

    #[napi]
    pub fn call_test_2(&mut self, func_name: String, data: Buffer) {
        let bytes: Vec<u8> = data.into();
        let chunks = bytes.chunks(16);

        let mut raw_values: Vec<RawValue> = Vec::new();
        for chunk in chunks {
            if chunk.len() < 16 {
                let mut padded_chunk = vec![0; 16];
                padded_chunk[..chunk.len()].copy_from_slice(chunk);
                raw_values.push(RawValue {
                    bytes: padded_chunk.try_into().unwrap(),
                });
            } else {
                raw_values.push(RawValue {
                    bytes: chunk.try_into().unwrap(),
                });
            }
        }

        self.contract.call_raw(&func_name, raw_values).unwrap();
    }
}
