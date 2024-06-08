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
