use chrono::Local;
use napi::{Env, Error, Task};
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
            func_name: func_name.to_owned(),
            wasm_params: wasm_params.to_vec(),
            start_time,
        }
    }
}

impl Task for ContractCallTask {
    type Output = Box<[Value]>;
    type JsValue = CallResponse;

    fn compute(&mut self) -> napi::Result<Self::Output> {
        // 1. Acquire the lock just long enough to check or set re-entrancy.
        {
            let mut svc = self.contract.lock().map_err(|_| {
                Error::from_reason("Failed to lock ContractService (poisoned)".to_string())
            })?;

            if svc.is_executing() {
                // Another call is in progress on this same contract => re-entrancy
                return Err(Error::from_reason(
                    "Re-entrancy error: contract is already executing".to_string(),
                ));
            }

            // Mark it as executing
            svc.set_executing(true);
        }

        // 2. Now do the WASM call WITHOUT holding the contract lock.
        let result = {
            // We only lock again for the actual call. If the Wasm code
            // triggers TSFN callbacks that in turn re-enter, we can
            // check is_executing again in the new code paths, or
            // return an error to prevent a deadlock.
            let mut svc = self.contract.lock().map_err(|_| {
                Error::from_reason("Failed to lock ContractService (poisoned)".to_string())
            })?;
            svc.call(&self.func_name, &self.wasm_params)
        };

        // 3. Release the "executing" flag after the call
        {
            let mut svc = self.contract.lock().map_err(|_| {
                Error::from_reason("Failed to lock ContractService (poisoned)".to_string())
            })?;
            svc.set_executing(false);
        }

        // Convert result to napi::Result
        result.map_err(|e| Error::from_reason(format!("{:?}", e)))
    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> napi::Result<Self::JsValue> {
        // Convert `Box<[Value]>` to a JS array
        let js_array = JsContract::box_values_to_js_array(&env, output)?;

        // Retrieve gas usage
        let gas_used = {
            let mut svc = self.contract.lock().map_err(|_| {
                Error::from_reason("Failed to lock ContractService (poisoned)".to_string())
            })?;
            svc.get_used_gas()
        };

        let gas_used_bigint = BigInt::from(gas_used);

        // Construct final response
        Ok(CallResponse {
            result: js_array,
            gas_used: gas_used_bigint,
        })
    }

    fn reject(&mut self, _env: Env, err: Error) -> napi::Result<Self::JsValue> {
        Err(err)
    }

    fn finally(&mut self, _env: Env) -> napi::Result<()> {
        // If needed, ensure re-entrancy guard is cleared in case of panic
        let mut svc = self.contract.lock().unwrap();
        svc.set_executing(false);
        Ok(())
    }
}
