use napi::{Env, Error, JsNumber, JsUnknown, Result};
use napi::bindgen_prelude::{Array, BigInt, Buffer, Undefined};
use wasmer::Value;

use crate::domain::contract::Contract;
use crate::domain::runner::WasmerInstance;
use crate::interfaces::{AbortDataResponse, CallResponse};

#[napi(js_name = "Contract")]
pub struct JsContract {
    contract: Contract,
}

#[napi] //noinspection RsCompileErrorMacro
impl JsContract {
    #[napi(constructor)]
    pub fn new(bytecode: Buffer, max_gas: BigInt) -> Self {
        let bytecode_vec = bytecode.to_vec();
        let max_gas = max_gas.get_u64().1;
        let runner = WasmerInstance::new(&bytecode_vec, max_gas);
        let contract = Contract::new(max_gas, Box::new(runner));
        Self { contract }
    }

    #[napi]
    pub fn call(&mut self, env: Env, func_name: String, params: Vec<JsNumber>) -> Result<CallResponse> {
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
        
        let gas_used = self.contract.get_used_gas();
        
        Ok(CallResponse {
            result: js_array,
            gas_used: BigInt::from(gas_used),
        })
    }

    #[napi]
    pub fn get_used_gas(&mut self) -> BigInt {
        let gas = self.contract.get_used_gas();
        BigInt::from(gas)
    }

    #[napi]
    pub fn set_used_gas(&mut self, gas: BigInt) {
        let gas = gas.get_u64().1;
        self.contract.set_used_gas(gas);
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
    pub fn get_abort_data(&self) -> Option<AbortDataResponse> {
        self.contract.get_abort_data().map(|data| data.into())
    }
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
