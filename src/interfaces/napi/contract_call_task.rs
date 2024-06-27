use std::sync::{Arc, Mutex};

use napi::{Env, Error, Task};
use napi::bindgen_prelude::BigInt;
use wasmer::Value;

use crate::domain::contract::Contract;
use crate::interfaces::CallResponse;
use crate::interfaces::napi::js_contract::JsContract;

pub struct ContractCallTask {
    contract: Arc<Mutex<Contract>>,
    func_name: String,
    wasm_params: Vec<Value>,
}

impl ContractCallTask {
    pub fn new(contract: Arc<Mutex<Contract>>, func_name: &str, wasm_params: &[Value]) -> Self {
        Self {
            contract,
            func_name: func_name.to_string(),
            wasm_params: wasm_params.to_vec(),
        }
    }
}

impl Task for ContractCallTask {
    type Output = Box<[Value]>;
    type JsValue = CallResponse;

    fn compute(&mut self) -> napi::Result<Self::Output> {
        let mut contract = self.contract.lock().unwrap();

        contract
            .call(&self.func_name, &self.wasm_params)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))
    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> napi::Result<Self::JsValue> {
        let js_array = JsContract::box_values_to_js_array(&env, output)?;
        let gas_used = self.contract.lock().unwrap().get_used_gas();

        let gas_used_bigint: BigInt = BigInt::from(gas_used);

        Ok(CallResponse {
            result: js_array,
            gas_used: gas_used_bigint,
        })
    }

    fn reject(&mut self, _env: Env, err: Error) -> napi::Result<Self::JsValue> {
        Err(err)
    }

    fn finally(&mut self, _env: Env) -> napi::Result<()> {
        Ok(())
    }
}
