use std::sync::{Arc, Mutex};

use chrono::Local;
use napi::bindgen_prelude::*;
use napi::bindgen_prelude::{Array, BigInt, Buffer, Undefined};
use napi::Env;
use napi::Error;
use napi::JsFunction;
use napi::JsNumber;
use napi::JsUnknown;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use wasmer::Value;

use crate::domain::contract::Contract;
use crate::domain::runner::WasmerInstance;
use crate::domain::vm::log_time_diff;
use crate::interfaces::{AbortDataResponse, CallOtherContractExternalFunction, ConsoleLogExternalFunction, ContractCallTask, DeployFromAddressExternalFunction, StorageLoadExternalFunction, StorageStoreExternalFunction};
use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;

macro_rules! create_tsfn {
    ($id:ident) => {
        $id.create_threadsafe_function(10, |ctx| Ok(vec![ctx.value]))?
    };
}

macro_rules! abort_tsfn {
    ($id:expr) => {
        if !$id.aborted() {
            $id.clone().abort()?;
        }
    };
}

#[napi(js_name = "Contract")]
pub struct JsContract {
    contract: Arc<Mutex<Contract>>,
    storage_load_tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    storage_store_tsfn:
        ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    call_other_contract_tsfn:
        ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    deploy_from_address_tsfn:
        ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    console_log_tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
}

#[napi] //noinspection RsCompileErrorMacro
impl JsContract {
    #[napi(constructor)]
    pub fn new(
        bytecode: Buffer,
        max_gas: BigInt,
        #[napi(
            ts_arg_type = "(_: never, result: Array<number>) => Promise<ThreadSafeJsImportResponse>"
        )]
        storage_load_js_function: JsFunction,
        #[napi(
            ts_arg_type = "(_: never, result: Array<number>) => Promise<ThreadSafeJsImportResponse>"
        )]
        storage_store_js_function: JsFunction,
        #[napi(
            ts_arg_type = "(_: never, result: Array<number>) => Promise<ThreadSafeJsImportResponse>"
        )]
        call_other_contract_js_function: JsFunction,
        #[napi(
            ts_arg_type = "(_: never, result: Array<number>) => Promise<ThreadSafeJsImportResponse>"
        )]
        deploy_from_address_js_function: JsFunction,
        #[napi(
            ts_arg_type = "(_: never, result: Array<number>) => Promise<ThreadSafeJsImportResponse>"
        )]
        console_log_js_function: JsFunction,
    ) -> Result<Self> {
        let time = Local::now();
        let bytecode_vec = bytecode.to_vec();
        let max_gas = max_gas.get_u64().1;

        let storage_load_tsfn = create_tsfn!(storage_load_js_function);
        let storage_store_tsfn = create_tsfn!(storage_store_js_function);
        let call_other_contract_tsfn = create_tsfn!(call_other_contract_js_function);
        let deploy_from_address_tsfn = create_tsfn!(deploy_from_address_js_function);
        let console_log_tsfn = create_tsfn!(console_log_js_function);

        let storage_load_external = StorageLoadExternalFunction::new(storage_load_tsfn.clone());
        let storage_store_external = StorageStoreExternalFunction::new(storage_store_tsfn.clone());
        let call_other_contract_external =
            CallOtherContractExternalFunction::new(call_other_contract_tsfn.clone());
        let deploy_from_address_external =
            DeployFromAddressExternalFunction::new(deploy_from_address_tsfn.clone());
        let console_log_external = ConsoleLogExternalFunction::new(console_log_tsfn.clone());

        let runner = WasmerInstance::new(
            &bytecode_vec,
            max_gas,
            storage_load_external,
            storage_store_external,
            call_other_contract_external,
            deploy_from_address_external,
            console_log_external,
        )
        .map_err(|e| Error::from_reason(format!("{:?}", e)))?;

        let runner = Arc::new(Mutex::new(runner));
        let contract = Contract::new(max_gas, runner);

        log_time_diff(&time, "JsContract::new");

        Ok(Self {
            contract: Arc::new(Mutex::new(contract)),
            storage_load_tsfn: storage_load_tsfn.clone(),
            storage_store_tsfn: storage_store_tsfn.clone(),
            call_other_contract_tsfn: call_other_contract_tsfn.clone(),
            deploy_from_address_tsfn: deploy_from_address_tsfn.clone(),
            console_log_tsfn: console_log_tsfn.clone(),
        })
    }

    #[napi]
    pub fn destroy(&self) -> Result<()> {
        abort_tsfn!(self.storage_load_tsfn);
        abort_tsfn!(self.storage_store_tsfn);
        abort_tsfn!(self.call_other_contract_tsfn);
        abort_tsfn!(self.deploy_from_address_tsfn);
        abort_tsfn!(self.console_log_tsfn);

        Ok(())
    }

    #[napi(ts_return_type = "Promise<CallResponse>")]
    pub fn call(
        &self,
        func_name: String,
        params: Vec<JsNumber>,
    ) -> Result<AsyncTask<ContractCallTask>> {
        let time = Local::now();
        let mut wasm_params = Vec::new();
        let length = params.len();

        for i in 0..length {
            let param = params.get(i).expect("Failed to get param");
            let param_value = param.get_int32();

            if param_value.is_err() {
                return Err(Error::from_reason(format!(
                    "{:?}",
                    param_value.unwrap_err()
                )));
            }

            wasm_params.push(Value::I32(param_value.unwrap()));
        }

        let contract = self.contract.clone();
        let result = AsyncTask::new(ContractCallTask::new(
            contract,
            &func_name,
            &wasm_params,
            time,
        ));

        Ok(result)
    }

    #[napi]
    pub fn read_memory(&self, offset: BigInt, length: BigInt) -> Result<Buffer> {
        let offset = offset.get_u64().1;
        let length = length.get_u64().1;
        let contract = self.contract.clone();

        let result = {
            let contract = contract.lock().unwrap();
            contract.read_memory(offset, length)
        };

        let resp = result.unwrap();

        Ok(Buffer::from(resp))
    }

    #[napi]
    pub fn write_memory(&self, offset: BigInt, data: Buffer) -> Result<Undefined> {
        let data: Vec<u8> = data.into();
        let offset = offset.get_u64().1;
        let contract = self.contract.clone();

        let contract = contract.lock().unwrap();
        contract.write_memory(offset, &data).unwrap();

        Ok(())
    }

    #[napi]
    pub fn get_used_gas(&self) -> Result<BigInt> {
        let contract = self.contract.clone();
        let gas = {
            let mut contract = contract.lock().unwrap();
            contract.get_used_gas()
        };

        Ok(BigInt::from(gas))
    }

    #[napi]
    pub fn set_used_gas(&self, gas: BigInt) -> Result<()> {
        let gas = gas.get_u64().1;
        let contract = self.contract.clone();
        let mut contract = contract.lock().unwrap();
        contract.set_used_gas(gas);

        Ok(())
    }

    #[napi]
    pub fn write_buffer(&self, value: Buffer, id: i32, align: u32) -> Result<i64> {
        let value = value.to_vec();
        let contract = self.contract.clone();

        let result = {
            let mut contract = contract.lock().unwrap();
            contract.write_buffer(&value, id, align)?
        };

        Ok(result)
    }

    #[napi]
    pub fn get_abort_data(&self) -> Option<AbortDataResponse> {
        let contract = self.contract.clone();
        let result = {
            let contract = contract.lock().unwrap();
            contract.get_abort_data().map(|data| data.into())
        };

        result
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

    pub(crate) fn box_values_to_js_array(env: &Env, values: Box<[Value]>) -> Result<Array> {
        let vals: Vec<Value> = values.clone().into_vec();
        let mut js_array = env.create_array(vals.len() as u32).unwrap();

        for value in values.iter() {
            let js_value = JsContract::value_to_js(env, value).unwrap();
            let _ = js_array.insert(js_value);
        }

        Ok(js_array)
    }
}
