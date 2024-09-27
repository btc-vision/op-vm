use crate::interfaces::napi::bitcoin_network_request::BitcoinNetworkRequest;
use crate::interfaces::napi::contract::JsContractParameter;
use crate::interfaces::napi::js_contract::JsContract;
use crate::interfaces::napi::runtime_pool::RuntimePool;
use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;
use crate::interfaces::{AbortDataResponse, ContractCallTask};
use anyhow::anyhow;
use bytes::Bytes;
use napi::bindgen_prelude::{AsyncTask, BigInt, Buffer, Undefined};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use napi::Env;
use napi::{Error, JsFunction, JsNumber};
use std::collections::HashMap;
use std::sync::Arc;

macro_rules! create_tsfn {
    ($id:ident) => {
        $id.create_threadsafe_function(10, |ctx| Ok(vec![ctx.value]))?
    };
}

macro_rules! abort_tsfn {
    ($id:expr, $env:expr) => {
        if !$id.aborted() {
            $id.clone().abort()?;
        }

        $id.unref(&$env)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
    };
}

#[napi(js_name = "ContractManager")]
pub struct ContractManager {
    contracts: HashMap<u64, JsContract>,
    contract_cache: HashMap<String, Bytes>,
    next_id: u64,
    #[napi(skip)]
    pub runtime_pool: Arc<RuntimePool>,
    #[napi(skip)]
    pub storage_load_tsfn:
        ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    #[napi(skip)]
    pub storage_store_tsfn:
        ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    #[napi(skip)]
    pub call_other_contract_tsfn:
        ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    #[napi(skip)]
    pub deploy_from_address_tsfn:
        ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    #[napi(skip)]
    pub console_log_tsfn:
        ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    #[napi(skip)]
    pub flush_events_tsfn:
        ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
}

#[napi]
impl ContractManager {
    #[napi(constructor)]
    pub fn new(
        max_idling_runtimes: u32,
        #[napi(
            ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer | Uint8Array>"
        )]
        storage_load_js_function: JsFunction,
        #[napi(
            ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer | Uint8Array>"
        )]
        storage_store_js_function: JsFunction,
        #[napi(
            ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer | Uint8Array>"
        )]
        call_other_contract_js_function: JsFunction,
        #[napi(
            ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer | Uint8Array>"
        )]
        deploy_from_address_js_function: JsFunction,
        #[napi(ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<void>")]
        console_log_js_function: JsFunction,
        #[napi(ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<void>")]
        flush_events_js_function: JsFunction,
    ) -> Result<Self, Error> {
        let storage_load_tsfn = create_tsfn!(storage_load_js_function);
        let storage_store_tsfn = create_tsfn!(storage_store_js_function);
        let call_other_contract_tsfn = create_tsfn!(call_other_contract_js_function);
        let deploy_from_address_tsfn = create_tsfn!(deploy_from_address_js_function);
        let console_log_tsfn = create_tsfn!(console_log_js_function);
        let flush_events_tsfn = create_tsfn!(flush_events_js_function);

        let max_idling_runtimes = max_idling_runtimes as usize;

        let runtime_pool = Arc::new(RuntimePool::new(max_idling_runtimes)); // 100 runtimes

        Ok(ContractManager {
            contracts: HashMap::new(),
            contract_cache: HashMap::new(),
            next_id: 1, // Start the ID counter at 1 (or 0, if preferred)
            storage_load_tsfn,
            storage_store_tsfn,
            call_other_contract_tsfn,
            deploy_from_address_tsfn,
            console_log_tsfn,
            runtime_pool,
            flush_events_tsfn,
        })
    }

    #[napi]
    pub fn reserve_id(&mut self) -> BigInt {
        let id = self.increment_next_id();

        BigInt::from(id)
    }

    #[napi]
    pub fn instantiate(
        &mut self,
        reserved_id: BigInt,
        address: String,
        bytecode: Option<Buffer>,
        max_gas: BigInt,
        network: BitcoinNetworkRequest,
    ) -> Result<(), Error> {
        let max_gas = max_gas.get_u64().1;
        let id = reserved_id.get_u64().1;

        let mut params: JsContractParameter = JsContractParameter {
            bytecode: None,
            serialized: None,
            max_gas,
            network,
        };

        let mut should_cache: bool = false;
        if self.contract_cache.contains_key(&address) {
            let serialized = self
                .contract_cache
                .get(&address)
                .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
            params.serialized = Some(serialized.clone());
        } else {
            let bytecode = bytecode
                .ok_or_else(|| Error::from_reason(anyhow!("Bytecode is required").to_string()))?
                .to_vec();

            should_cache = true;
            params.bytecode = Some(bytecode);
        }

        let js_contract: JsContract = JsContract::from(params, self, id)?;
        if should_cache {
            let serialized = js_contract.serialize()?;
            self.contract_cache.insert(address, serialized);
        }

        self.add_contract(id, js_contract)?;
        Ok(())
    }

    #[napi]
    pub fn validate_bytecode(&self, bytecode: Buffer, max_gas: BigInt) -> Result<bool, Error> {
        JsContract::validate_bytecode(bytecode, max_gas)
    }

    #[napi]
    pub fn destroy_contract(&mut self, id: BigInt) -> Result<bool, Error> {
        let id = id.get_u64().1;

        match self.contracts.remove(&id) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    #[napi]
    pub fn destroy(&mut self, env: Env) -> Result<(), Error> {
        abort_tsfn!(self.storage_load_tsfn, &env);
        abort_tsfn!(self.storage_store_tsfn, &env);
        abort_tsfn!(self.call_other_contract_tsfn, &env);
        abort_tsfn!(self.deploy_from_address_tsfn, &env);
        abort_tsfn!(self.console_log_tsfn, &env);

        //self.runtime_pool.destroy();

        Ok(())
    }

    #[napi]
    pub fn destroy_cache(&mut self) -> () {
        self.contract_cache.clear();

        ()
    }

    #[napi]
    pub fn destroy_all(&mut self) -> () {
        self.contracts.clear();
        self.contract_cache.clear();

        ()
    }

    fn increment_next_id(&mut self) -> u64 {
        if self.next_id > u64::MAX - 1 {
            self.next_id = 1;
        }

        let id = self.next_id;
        self.next_id += 1;

        id
    }

    // Add a JsContract to the map and return its ID
    fn add_contract(&mut self, id: u64, contract: JsContract) -> Result<u64, Error> {
        self.contracts.insert(id, contract);

        Ok(id)
    }

    #[napi]
    pub fn use_gas(&self, contract_id: BigInt, gas: BigInt) -> Result<(), Error> {
        let id = contract_id.get_u64().1;

        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.use_gas(gas)
    }

    #[napi]
    pub fn write_buffer(
        &self,
        contract_id: BigInt,
        value: Buffer,
        id: i32,
        align: u32,
    ) -> Result<i64, Error> {
        let contract_id = contract_id.get_u64().1;

        let contract = self
            .contracts
            .get(&contract_id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.write_buffer(value, id, align)
    }

    #[napi]
    pub fn get_abort_data(&self, contract_id: BigInt) -> Result<AbortDataResponse, Error> {
        let id = contract_id.get_u64().1;

        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.get_abort_data()
    }

    #[napi]
    pub fn set_remaining_gas(&self, id: BigInt, gas: BigInt) -> Result<(), Error> {
        let id = id.get_u64().1;

        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.set_remaining_gas(gas)
    }

    #[napi]
    pub fn get_remaining_gas(&self, id: BigInt) -> Result<BigInt, Error> {
        let id = id.get_u64().1;

        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.get_remaining_gas()
    }

    #[napi]
    pub fn set_used_gas(&self, id: BigInt, gas: BigInt) -> Result<(), Error> {
        let id = id.get_u64().1;

        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.set_used_gas(gas)
    }

    #[napi]
    pub fn get_used_gas(&self, id: BigInt) -> Result<BigInt, Error> {
        let id = id.get_u64().1;

        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.get_used_gas()
    }

    #[napi]
    pub fn write_memory(
        &self,
        id: BigInt,
        offset: BigInt,
        data: Buffer,
    ) -> Result<Undefined, Error> {
        let id = id.get_u64().1;

        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.write_memory(offset, data)
    }

    #[napi]
    pub fn read_memory(&self, id: BigInt, offset: BigInt, length: BigInt) -> Result<Buffer, Error> {
        let id = id.get_u64().1;

        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.read_memory(offset, length)
    }

    #[napi(ts_return_type = "Promise<CallResponse>")]
    pub fn call(
        &self,
        id: BigInt,
        func_name: String,
        params: Vec<JsNumber>,
    ) -> Result<AsyncTask<ContractCallTask>, Error> {
        let id = id.get_u64().1;

        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        let result = contract.call(func_name, params)?;

        Ok(result)
    }

    #[napi]
    pub fn length(&self) -> Result<BigInt, Error> {
        Ok(BigInt::from(self.contracts.len() as u64))
    }

    #[napi]
    pub fn clear(&mut self) -> Result<(), Error> {
        //for contract in self.contracts.values_mut() {
        //    contract.destroy(env)?;
        //}

        self.contracts.clear();

        Ok(())
    }
}
