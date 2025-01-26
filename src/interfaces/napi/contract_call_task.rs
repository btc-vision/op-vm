use chrono::Local;
use napi::{Env, Error, Result as NapiResult, Task};
use std::sync::{Arc, Mutex};
use wasmer::Value;

use crate::application::contract::ContractService;
use crate::interfaces::napi::js_contract::JsContract;
use crate::interfaces::CallResponse;
use napi::bindgen_prelude::BigInt;

pub struct ContractCallTask {
    contract: Arc<Mutex<ContractService>>,
    func_name: String,
    wasm_params: Vec<Value>,
    start_time: chrono::DateTime<Local>,
}

impl ContractCallTask {
    pub fn new(
        contract: Arc<Mutex<ContractService>>,
        func_name: &str,
        wasm_params: &[Value],
        start_time: chrono::DateTime<Local>,
    ) -> Self {
        Self {
            contract,
            func_name: func_name.into(),
            wasm_params: wasm_params.to_vec(),
            start_time,
        }
    }
}

#[napi]
impl Task for ContractCallTask {
    type Output = Box<[Value]>;
    type JsValue = CallResponse;

    fn compute(&mut self) -> NapiResult<Self::Output> {
        {
            let mut svc = self
                .contract
                .lock()
                .map_err(|_| Error::from_reason("Lock ContractService (poisoned)".to_string()))?;

            if svc.is_executing() {
                return Err(Error::from_reason("Re-entrancy error".to_string()));
            }
            svc.set_executing(true);
        }

        let result = {
            let mut svc = self
                .contract
                .lock()
                .map_err(|_| Error::from_reason("Lock ContractService (poisoned)".to_string()))?;
            svc.call(&self.func_name, &self.wasm_params)
        };

        {
            let mut svc = self
                .contract
                .lock()
                .map_err(|_| Error::from_reason("Lock ContractService (poisoned)".to_string()))?;
            svc.set_executing(false);
        }

        result.map_err(|e| Error::from_reason(format!("{:?}", e)))
    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> NapiResult<Self::JsValue> {
        let js_array = JsContract::box_values_to_js_array(&env, output)?;
        let gas_used = {
            let mut svc = self
                .contract
                .lock()
                .map_err(|_| Error::from_reason("Lock ContractService (poisoned)".to_string()))?;
            svc.get_used_gas()
        };

        Ok(CallResponse {
            result: js_array,
            gas_used: BigInt::from(gas_used),
        })
    }

    fn reject(&mut self, _env: Env, err: Error) -> NapiResult<Self::JsValue> {
        Err(err)
    }

    fn finally(self, _env: Env) -> NapiResult<()> {
        // ensure re-entrancy is cleared if not already
        if let Ok(mut svc) = self.contract.lock() {
            svc.set_executing(false);
        }
        Ok(())
    }
}
