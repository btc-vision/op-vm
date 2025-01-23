use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use bytes::Bytes;
// For alpha.27, we'll do NapiResult alias or just use `napi::Result`
use napi::bindgen_prelude::Buffer;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::{
    bindgen_prelude::{AsyncTask, BigInt, Function, Undefined, Unknown},
    Error, JsNumber, Result as NapiResult,
};
use napi_derive::napi;

use crate::interfaces::napi::{
    bitcoin_network_request::BitcoinNetworkRequest, contract::JsContractParameter,
    js_contract::JsContract, runtime_pool::RuntimePool,
    thread_safe_js_import_response::ThreadSafeJsImportResponse,
};
use crate::interfaces::{AbortDataResponse, ContractCallTask};

/// Type alias for a TSFN that sends a `ThreadSafeJsImportResponse`
/// into JavaScript and receives a `JsUnknown` result back.
///
/// The JS side *can* resolve the Promise to a Buffer, but from Rust’s
/// perspective we will see a `JsUnknown` that we can parse as Buffer.
pub type TsfnBuffer = ThreadsafeFunction<
    ThreadSafeJsImportResponse,    // T: the data we pass to JS
    Buffer,                        // Return: the JS return type is `JsUnknown` in 3.0 alpha
    (ThreadSafeJsImportResponse,), // The "CallJsBackArgs" we pass into JS. A single argument tuple
    true,                          // calleeHandled
    false,                         // Weak
    10,                            // max_queue_size
>;

/// Type alias for a TSFN that sends a `ThreadSafeJsImportResponse`
/// into JavaScript and expects "void" from JS.
/// (In 3.0 alpha, that’s still `JsUnknown`, we just ignore it.)
pub type TsfnVoid = ThreadsafeFunction<
    ThreadSafeJsImportResponse,
    Unknown,
    (ThreadSafeJsImportResponse,),
    true,
    false,
    10,
>;

/// Macro to create a TSFN that we *intend* to treat like
/// "Promise<Buffer or Uint8Array>" in JavaScript. But in 3.0 alpha,
/// the actual `Return` is `JsUnknown`. We call it "TsfnBufferLike".
#[macro_export]
macro_rules! create_tsfn_buffer {
    ($js_func:expr) => {{
        // 1) We pick the "T" type for build_threadsafe_function<T>()
        //    In your old code, T was `ThreadSafeJsImportResponse`.
        let builder = $js_func.build_threadsafe_function::<ThreadSafeJsImportResponse>();

        // 2) calleeHandled + max_queue_size
        let builder = builder.callee_handled::<true>();
        let builder = builder.max_queue_size::<10>();

        // 3) We next call `.build_callback<CallJsBackArgs, _>(...)`
        //    so that we pass `(ThreadSafeJsImportResponse,)` to JS as the argument list.
        //
        //    The closure returns `Ok((ctx.value,))`, meaning we pass exactly one argument
        //    (the data that was passed from Rust).
        //    The final TSFN type is: ThreadsafeFunction<ThreadSafeJsImportResponse,
        //       JsUnknown, (ThreadSafeJsImportResponse,), true, false, 10>
        //
        //    We'll call it "TsfnBufferLike" just for our own alias.
        let tsfn = builder.build_callback::<(ThreadSafeJsImportResponse,), _>(|ctx| {
            // pass one argument => (ctx.value,)
            Ok((ctx.value,))
        })?;

        // Wrap in Arc if you want to store it
        std::sync::Arc::new(tsfn)
    }};
}

/// Macro to create a TSFN that we interpret as "Promise<void>",
/// i.e. we only pass `ThreadSafeJsImportResponse` in, and ignore the result.
#[macro_export]
macro_rules! create_tsfn_void {
    ($js_func:expr) => {{
        // T is the data we pass to JS:
        let builder = $js_func.build_threadsafe_function::<ThreadSafeJsImportResponse>();
        let builder = builder.callee_handled::<true>();
        let builder = builder.max_queue_size::<10>();

        // The closure returns a single argument tuple
        // and the final result is also `JsUnknown`.
        let tsfn =
            builder.build_callback::<(ThreadSafeJsImportResponse,), _>(|ctx| Ok((ctx.value,)))?;

        std::sync::Arc::new(tsfn)
    }};
}

#[napi(js_name = "ContractManager")]
pub struct ContractManager {
    contracts: HashMap<u64, JsContract>,
    contract_cache: HashMap<String, Bytes>,
    next_id: u64,

    pub runtime_pool: Arc<RuntimePool>,
    pub storage_load_tsfn: Arc<TsfnBuffer>,
    pub storage_store_tsfn: Arc<TsfnBuffer>,
    pub call_other_contract_tsfn: Arc<TsfnBuffer>,
    pub deploy_from_address_tsfn: Arc<TsfnBuffer>,
    pub console_log_tsfn: Arc<TsfnVoid>,
    pub emit_tsfn: Arc<TsfnVoid>,
    pub inputs_tsfn: Arc<TsfnBuffer>,
    pub outputs_tsfn: Arc<TsfnBuffer>,
    pub next_pointer_value_greater_than_tsfn: Arc<TsfnBuffer>,
}

#[napi]
impl ContractManager {
    #[napi(constructor)]
    pub fn new(
        max_idling_runtimes: u32,
        storage_load_js_function: Function,
        storage_store_js_function: Function,
        call_other_contract_js_function: Function,
        deploy_from_address_js_function: Function,
        console_log_js_function: Function,
        emit_js_function: Function,
        inputs_js_function: Function,
        outputs_js_function: Function,
        next_pointer_value_greater_than_js_function: Function,
    ) -> Result<Self, Error> {
        let storage_load_tsfn = create_tsfn_buffer!(storage_load_js_function);
        let storage_store_tsfn = create_tsfn_buffer!(storage_store_js_function);
        let call_other_contract_tsfn = create_tsfn_buffer!(call_other_contract_js_function);
        let deploy_from_address_tsfn = create_tsfn_buffer!(deploy_from_address_js_function);
        let console_log_tsfn = create_tsfn_void!(console_log_js_function);
        let emit_tsfn = create_tsfn_void!(emit_js_function);
        let inputs_tsfn = create_tsfn_buffer!(inputs_js_function);
        let outputs_tsfn = create_tsfn_buffer!(outputs_js_function);
        let next_pointer_value_greater_than_tsfn =
            create_tsfn_buffer!(next_pointer_value_greater_than_js_function);

        let max_idling_runtimes = max_idling_runtimes as usize;
        let runtime_pool = Arc::new(RuntimePool::new(max_idling_runtimes));

        Ok(Self {
            contracts: HashMap::new(),
            contract_cache: HashMap::new(),
            next_id: 1,
            runtime_pool,
            storage_load_tsfn,
            storage_store_tsfn,
            call_other_contract_tsfn,
            deploy_from_address_tsfn,
            console_log_tsfn,
            emit_tsfn,
            inputs_tsfn,
            outputs_tsfn,
            next_pointer_value_greater_than_tsfn,
        })
    }

    #[napi]
    pub fn destroy(&mut self) -> NapiResult<()> {
        Ok(())
    }

    #[napi]
    pub fn destroy_cache(&mut self) {
        self.contract_cache.clear();
    }

    #[napi]
    pub fn destroy_all(&mut self) {
        self.contracts.clear();
        self.contract_cache.clear();
    }

    #[napi]
    pub fn reserve_id(&mut self) -> BigInt {
        let id = self.increment_next_id();
        BigInt::from(id)
    }

    fn increment_next_id(&mut self) -> u64 {
        if self.next_id >= u64::MAX {
            self.next_id = 1;
        }
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    #[napi]
    pub fn instantiate(
        &mut self,
        reserved_id: BigInt,
        address: String,
        bytecode: Option<Buffer>,
        max_gas: BigInt,
        network: BitcoinNetworkRequest,
    ) -> NapiResult<()> {
        let max_gas = max_gas.get_u64().1;
        let id = reserved_id.get_u64().1;

        let mut params = JsContractParameter {
            bytecode: None,
            serialized: None,
            max_gas,
            network,
        };

        let mut should_cache = false;
        if let Some(serialized) = self.contract_cache.get(&address) {
            params.serialized = Some(serialized.clone());
        } else {
            let bc = bytecode
                .ok_or_else(|| Error::from_reason(anyhow!("Bytecode is required").to_string()))?
                .to_vec();
            should_cache = true;
            params.bytecode = Some(bc);
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

    fn add_contract(&mut self, id: u64, contract: JsContract) -> NapiResult<u64> {
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
