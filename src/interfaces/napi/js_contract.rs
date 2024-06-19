use napi::{Env, Error, JsNumber, JsUnknown, Result};
use napi::bindgen_prelude::{Array, BigInt, Buffer, Undefined};
use wasmer::Value;
use wasmer_types::RawValue;

use crate::domain::contract::Contract;
use crate::domain::runner::WasmerInstance;
use crate::interfaces::AbortDataResponse;

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
}

#[napi] //noinspection RsCompileErrorMacro
impl JsContract {
    #[napi(constructor)]
    pub fn new(bytecode: Buffer) -> Self {
        let bytecode_vec = bytecode.to_vec();
        let runner = WasmerInstance::new(&bytecode_vec);
        let contract = Contract::new(Box::new(runner));
        Self { contract }
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
    pub fn write_buffer(&mut self, value: Buffer, id: i32, align: u32) -> Result<i64> {
        let value = value.to_vec();
        self.contract.write_buffer(&value, id, align)
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

        let result = self
            .contract
            .call(&func_name, &wasm_params)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
        let js_array = JsContract::box_values_to_js_array(&env, result)?;

        Ok(js_array)
    }

    #[napi]
    pub fn get_abort_data(&self) -> Option<AbortDataResponse> {
        self.contract.get_abort_data().map(|data| data.into())
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
