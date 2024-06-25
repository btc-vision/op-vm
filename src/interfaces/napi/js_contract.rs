use std::sync::{Arc, mpsc, Mutex};
use std::thread;

use napi::bindgen_prelude::*;
use napi::bindgen_prelude::{Array, BigInt, Buffer, Undefined};
use napi::Env;
use napi::Error;
use napi::JsFunction;
use napi::JsNumber;
use napi::JsUnknown;
use napi::Result;
use napi::threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction};
use wasmer::Value;

use crate::domain::contract::Contract;
use crate::domain::runner::WasmerInstance;
use crate::interfaces::{AbortDataResponse, CallResponse};

#[napi(js_name = "Contract")]
pub struct JsContract {
    contract: Arc<Mutex<Contract>>,
    deploy_tsfn: ThreadsafeFunction<Vec<u8>, ErrorStrategy::CalleeHandled>,
}

pub struct ContractCallTask {
    contract: Arc<Mutex<Contract>>,
    func_name: String,
    wasm_params: Vec<Value>,
}

impl Task for ContractCallTask {
    type Output = Box<[Value]>;
    type JsValue = CallResponse;

    fn compute(&mut self) -> Result<Self::Output> {
        let mut contract = self.contract.lock().unwrap();

        contract.call(&self.func_name, &self.wasm_params).map_err(|e| Error::from_reason(format!("{:?}", e)))
    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        let js_array = JsContract::box_values_to_js_array(&env, output)?;
        let gas_used = self.contract.lock().unwrap().get_used_gas();

        let gas_used_bigint: BigInt = BigInt::from(gas_used);

        Ok(CallResponse {
            result: js_array,
            gas_used: gas_used_bigint,
        })
    }

    fn reject(&mut self, _env: Env, err: Error) -> Result<Self::JsValue> {
        Err(err)
    }

    fn finally(&mut self, _env: Env) -> Result<()> {
        Ok(())
    }
}

#[napi] //noinspection RsCompileErrorMacro
impl JsContract {
    #[napi(constructor)]
    pub fn new(
        bytecode: Buffer,
        max_gas: BigInt,
        js_load_function: JsFunction,
    ) -> Result<Self> {
        let bytecode_vec = bytecode.to_vec();
        let max_gas = max_gas.get_u64().1;

        let tsfn: ThreadsafeFunction<Vec<u8>, ErrorStrategy::CalleeHandled> = js_load_function
            .create_threadsafe_function(10, move |ctx: ThreadSafeCallContext<Vec<u8>>| {
                Ok(vec![ctx.value])
            })?;

        let (tx, rx) = mpsc::channel();

        let deploy_tsfn_clone = tsfn.clone();
        let handle = thread::spawn(move || {
            let runner = WasmerInstance::new(&bytecode_vec, max_gas, deploy_tsfn_clone)
                .map_err(|e| Error::from_reason(format!("{:?}", e)))
                .unwrap();

            let runner = Arc::new(Mutex::new(runner));
            let contract = Contract::new(max_gas, runner);

            // Send the contract back to the main thread
            tx.send(contract).expect("Failed to send contract");
        });

        let contract = rx.recv().expect("Failed to receive contract");

        handle.join().expect("Thread panicked");

        let deploy_tsfn_clone = tsfn.clone();
        Ok(Self { contract: Arc::new(Mutex::new(contract)), deploy_tsfn: deploy_tsfn_clone })
    }

    #[napi]
    pub fn call(&self, func_name: String, params: Vec<JsNumber>) -> Result<AsyncTask<ContractCallTask>> {
        let mut wasm_params = Vec::new();
        let length = params.len();

        for i in 0..length {
            let param = params.get(i).expect("Failed to get param");
            let param_value = param.get_int32();

            if param_value.is_err() {
                return Err(Error::from_reason(format!("{:?}", param_value.unwrap_err())));
            }

            wasm_params.push(Value::I32(param_value.unwrap()));
        }

        let contract = self.contract.clone();
        Ok(AsyncTask::new(ContractCallTask {
            contract,
            func_name,
            wasm_params,
        }))
    }

    #[napi]
    pub fn read_memory(&self, offset: BigInt, length: BigInt) -> Result<Buffer> {
        let (tx, rx) = mpsc::channel();
        let offset = offset.get_u64().1;
        let length = length.get_u64().1;
        let contract = self.contract.clone();

        thread::spawn(move || {
            let result = {
                let contract = contract.lock().unwrap();
                contract.read_memory(offset, length)
            };
            tx.send(result).expect("Failed to send read memory result");
        });

        let result = rx.recv().map_err(|e| Error::from_reason(format!("Recv error: {:?}", e)))?;
        let resp = result.unwrap();

        Ok(Buffer::from(resp))
    }

    #[napi]
    pub fn write_memory(&self, offset: BigInt, data: Buffer) -> Result<Undefined> {
        let (tx, rx) = mpsc::channel();
        let data: Vec<u8> = data.into();
        let offset = offset.get_u64().1;
        let contract = self.contract.clone();

        thread::spawn(move || {
            let result = {
                let contract = contract.lock().unwrap();
                contract.write_memory(offset, &data)
            };
            tx.send(result).expect("Failed to send write memory result");
        });

        rx.recv().map_err(|e| Error::from_reason(format!("Recv error: {:?}", e)))?.map_err(|e| Error::from_reason(format!("{:?}", e)))?;
        Ok(())
    }

    #[napi]
    pub fn destroy(&mut self) -> Result<()> {
        self.deploy_tsfn.clone().abort().expect("TODO: panic message");
        Ok(())
    }

    #[napi]
    pub fn get_used_gas(&self) -> Result<BigInt> {
        let (tx, rx) = mpsc::channel();
        let contract = self.contract.clone();
        thread::spawn(move || {
            let gas = {
                let mut contract = contract.lock().unwrap();
                contract.get_used_gas()
            };
            tx.send(gas).expect("Failed to send gas used");
        });

        let gas = rx.recv().map_err(|e| Error::from_reason(format!("Recv error: {:?}", e)))?;
        Ok(BigInt::from(gas))
    }

    #[napi]
    pub fn set_used_gas(&self, gas: BigInt) -> Result<()> {
        let gas = gas.get_u64().1;
        let contract = self.contract.clone();
        thread::spawn(move || {
            let mut contract = contract.lock().unwrap();
            contract.set_used_gas(gas);
        });

        Ok(())
    }

    #[napi]
    pub fn write_buffer(&self, value: Buffer, id: i32, align: u32) -> Result<i64> {
        let (tx, rx) = mpsc::channel();
        let value = value.to_vec();
        let contract = self.contract.clone();

        thread::spawn(move || {
            let result = {
                let mut contract = contract.lock().unwrap();
                contract.write_buffer(&value, id, align)
            };
            tx.send(result).expect("Failed to send write buffer result");
        });

        let result = rx.recv().map_err(|e| Error::from_reason(format!("Recv error: {:?}", e)))??;
        Ok(result)
    }

    #[napi]
    pub fn get_abort_data(&self) -> Option<AbortDataResponse> {
        let (tx, rx) = mpsc::channel();
        let contract = self.contract.clone();
        thread::spawn(move || {
            let result = {
                let contract = contract.lock().unwrap();
                contract.get_abort_data().map(|data| data.into())
            };
            tx.send(result).expect("Failed to send abort data result");
        });

        rx.recv().expect("Recv error")
    }

    /*#[napi]
    pub fn call(
        &mut self,
        env: Env,
        func_name: String,
        params: Vec<JsNumber>,
    ) -> Result<CallResponse> {
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
    }*/
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
