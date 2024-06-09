use std::sync::{Arc, Mutex};
use std::thread;

use napi::{Env, Error, JsBuffer, JsBufferValue, JsNumber, JsUnknown, Result};
use napi::bindgen_prelude::{Array, BigInt, Buffer, JsValuesTupleIntoVec, Undefined};
use wasmer::Value;

use crate::contract::Contract;

#[napi(js_name = "Contract")]
pub struct JsContract {
    contract: Arc<Mutex<Option<Contract>>>,
}

fn get_undefined() -> Undefined {
    ()
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
    #[napi(factory)]
    pub fn instanciate(bytecode: JsBuffer, address: String, deployer: String) -> Result<Self> {
        let bytecode_value: Result<JsBufferValue> = bytecode.into_value();

        match bytecode_value {
            Ok(bytecode) => {
                let vector_bytecode = bytecode.to_vec();
                let contract_result = thread::spawn(move || {
                    Contract::new(vector_bytecode, &address, &deployer)
                }).join();

                match contract_result {
                    Ok(contract) => {
                        match contract {
                            Ok(contract_instance) => Ok(JsContract { contract: Arc::new(Mutex::new(Some(contract_instance))) }),
                            Err(e) => Err(Error::from_reason(e)),
                        }
                    }
                    Err(e) => Err(Error::from_reason(format!("Thread panicked: {:?}", e))),
                }
            }
            Err(err) => Err(Error::from_reason(format!("Failed to convert JsBuffer to Vec<u8>: {:?}", err))),
        }
    }

    #[napi]
    pub fn read_memory(&mut self, env: Env, offset: BigInt, length: BigInt) -> Result<JsBuffer> {
        let contract = self.contract.clone();
        let mut contract = contract.lock().unwrap();

        let offset = offset.get_u64().1;
        let length = length.get_u64().1;

        if let Some(contract) = contract.as_mut() {
            let buffer = contract.read(offset, length);

            if buffer.is_err() {
                return Err(Error::from_reason(format!("{:?}", buffer.unwrap_err())));
            }

            let buffer = buffer.unwrap();
            let buf = env.create_buffer_with_data(buffer).unwrap().into_raw();

            return Ok(buf);
        }

        Err(Error::from_reason("Contract not initialized"))
    }

    #[napi]
    pub fn write_memory(&mut self, offset: BigInt, data: Buffer) -> Result<Undefined> {
        let contract = self.contract.clone();
        let mut contract = contract.lock().unwrap();

        let data: Vec<u8> = data.into();
        let offset = offset.get_u64().1;

        if let Some(contract) = contract.as_mut() {
            let written = contract.write(offset, &data);
            if written.is_err() {
                return Err(Error::from_reason(format!("{:?}", written.unwrap_err())));
            }

            return Ok(get_undefined());
        }

        Err(Error::from_reason("Contract not initialized"))
    }

    #[napi]
    pub fn init(&self) -> Result<Undefined> {
        let contract = self.contract.clone();
        let mut contract = contract.lock().unwrap();

        if let Some(contract) = contract.as_mut() {
            contract.init().map_err(|e| Error::from_reason(format!("{:?}", e)))?;
        }

        Ok(get_undefined())
    }

    #[napi]
    pub fn lower_string(&self, env: Env, value: String) -> Result<JsNumber> {
        let contract = self.contract.clone();
        let mut contract = contract.lock().unwrap();

        if let Some(contract) = contract.as_mut() {
            let result = contract.lower_string(&value).map_err(|e| Error::from_reason(format!("{:?}", e)))?;
            let js_number = env.create_int32(result as i32);

            return js_number;
        } else {
            Err(Error::from_reason("Contract not initialized"))
        }
    }

    #[napi]
    pub fn write_buffer(&self, env: Env, value: JsBuffer, id: JsNumber, align: u32) -> Result<JsNumber> {
        let contract = self.contract.clone();
        let mut contract = contract.lock().unwrap();

        // Convert JsBuffer to Vec<u8>
        let value = value.into_value()?.to_vec();

        // Convert JsNumber to i32
        let id = id.get_int32()?;

        // Ensure the contract is initialized
        if let Some(contract) = contract.as_mut() {
            // Calculate the length and create a new buffer
            let length = value.len();
            let buffer_size = length << align;
            let buffer = contract.__new(buffer_size as i32, 1);
            if buffer.is_err() {
                return Err(Error::from_reason(format!("Failed to get buffer from __new: {:?}", buffer.unwrap_err())));
            }

            let buffer_value = buffer.unwrap();

            // Pin the buffer
            let pinned_buffer = contract.__pin(buffer_value);
            if pinned_buffer.is_err() {
                return Err(Error::from_reason(format!("Failed to pin buffer: {:?}", pinned_buffer.unwrap_err())));
            }

            let pin_value: Value = pinned_buffer.unwrap()[0].clone();
            let pinned_buffer_value = pin_value.unwrap_i32() as u32;

            // Create the header
            let header = contract.__new(12, id);
            if header.is_err() {
                return Err(Error::from_reason(format!("Failed to get header from __new: {:?}", header.unwrap_err())));
            }

            let header_value = header.unwrap();

            // Set the header values
            let set = contract.set_u32(header_value, pinned_buffer_value);
            let set = contract.set_u32(header_value + 4, pinned_buffer_value);
            let set = contract.set_u32(header_value + 8, buffer_size as u32);

            // Write the buffer value to the contract's memory
            let set = contract.write_pointer(pinned_buffer_value as u64, value);

            // Unpin the buffer
            let set = contract.__unpin(pinned_buffer_value as i32);

            // Create a JsNumber from the header value
            let result_pointer = env.create_int64(header_value as i64);

            return result_pointer;
        }

        Err(Error::from_reason("Contract not initialized"))
    }

    #[napi]
    pub fn lift_typed_array(&self, env: Env, raw_offset: JsNumber) -> Result<JsBuffer> {
        let contract = self.contract.clone();
        let mut contract = contract.lock().unwrap();

        let offset = raw_offset.get_int32();
        if offset.is_err() {
            return Err(Error::from_reason("Failed to read offset"));
        }

        let offset = offset.unwrap();

        if let Some(contract) = contract.as_mut() {
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
            let buf = env.create_buffer_with_data(buffer).unwrap().into_raw();

            Ok(buf)
        } else {
            Err(Error::from_reason("Contract not initialized"))
        }
    }

    #[napi]
    pub fn call(&self, env: Env, func_name: String, params: Vec<JsNumber>) -> Result<Array> {
        let contract = self.contract.clone();
        let mut contract = contract.lock().unwrap();

        if let Some(contract) = contract.as_mut() {
            let mut wasm_params = Vec::new();

            for param in params {
                let param_value = param.get_int32();
                if param_value.is_err() {
                    return Err(Error::from_reason(format!("{:?}", param_value.unwrap_err())));
                }

                let value = param_value.unwrap();
                wasm_params.push(Value::I32(value));
            }

            let result = contract.call(&func_name, &wasm_params).map_err(|e| Error::from_reason(format!("{:?}", e)))?;
            let js_array = JsContract::box_values_to_js_array(&env, result)?;

            Ok(js_array)
        } else {
            Err(Error::from_reason("Contract not initialized"))
        }
    }
}
