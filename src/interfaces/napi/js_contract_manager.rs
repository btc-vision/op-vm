use crate::domain::runner;
use crate::domain::runner::ExitData;
use crate::interfaces::napi::bitcoin_network_request::BitcoinNetworkRequest;
use crate::interfaces::napi::contract::JsContractParameter;
use crate::interfaces::napi::environment_variables_request::EnvironmentVariablesRequest;
use crate::interfaces::napi::external_functions::BlockHashRequest;
use crate::interfaces::napi::js_contract::JsContract;
use crate::interfaces::napi::runtime_pool::RuntimePool;
use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;
use crate::interfaces::{AccountTypeResponse, JsBlockHashResponse};
use anyhow::anyhow;
use bytes::Bytes;
use napi::bindgen_prelude::{
    BigInt, Buffer, BufferSlice, Function, JsObjectValue, Object, Promise, PromiseRaw,
};
use napi::threadsafe_function::ThreadsafeFunction;
use napi::Error;
use napi::{Env, Status};
use std::collections::HashMap;
use std::sync::Arc;

#[napi]
#[allow(dead_code)]
pub const NEW_STORAGE_SLOT_GAS_COST: u64 = runner::NEW_STORAGE_SLOT_GAS_COST;

#[napi]
#[allow(dead_code)]
pub const UPDATED_STORAGE_SLOT_GAS_COST: u64 = runner::UPDATED_STORAGE_SLOT_GAS_COST;

/// Turns a `JsFunction` into a `ThreadsafeFunction` whose second tuple
/// element (the “response” coming from the JS side) is decided by the caller.
///
/// Usage:
///
/// ```rust
/// let tsfn = build_tsfn!(storage_load_js_function, ThreadSafeJsImportResponse);
/// let tsfn = build_tsfn!(storage_load_js_function, MyResponse, 256); // queue = 256
/// ```
#[macro_export]
macro_rules! build_tsfn {
  // main entry
  ( $fn_ident:expr, $resp:ty, $ret:ty $( , $queue:expr )? ) => {{
    const Q: usize = build_tsfn!(@queue $( $queue )?);
    let tsfn: Arc<ThreadsafeFunction<
      $resp,
      $ret,
      $resp,
      Status,
      true,
      false,
      Q,
    >> = Arc::new($fn_ident
      .build_threadsafe_function()
      .max_queue_size::<Q>()
      .callee_handled::<true>()
      .weak::<false>()
      .build()?);
    tsfn
  }};

  // handle the optional queue literal
  (@queue) => { 128 };
  (@queue $q:expr) => { $q };
}

/*pub struct IntArrayResponse {
    contract: Arc<JsContract>,
    values: Box<[Value]>,
}

impl ToNapiValue for IntArrayResponse {
    unsafe fn to_napi_value(env_raw: napi_env, val: Self) -> napi::Result<napi_value> {
        let env = Env::from_raw(env_raw);
        let js_array = val.contract.convert_values_to_js_array(&env, val.values)?;
        Ok(js_array.into_raw())
    }
}*/

#[napi(js_name = "ContractManager")]
pub struct ContractManager {
    contracts: HashMap<u64, Arc<JsContract>>,
    contract_cache: HashMap<String, Bytes>,
    next_id: u64,
    #[napi(skip)]
    pub runtime_pool: Arc<RuntimePool>,
    #[napi(skip)]
    pub storage_load_tsfn: Arc<
        ThreadsafeFunction<
            ThreadSafeJsImportResponse,
            Promise<Buffer>,
            ThreadSafeJsImportResponse,
            Status,
            true,
            false,
            128,
        >,
    >,
    #[napi(skip)]
    pub storage_store_tsfn: Arc<
        ThreadsafeFunction<
            ThreadSafeJsImportResponse,
            Promise<Buffer>,
            ThreadSafeJsImportResponse,
            Status,
            true,
            false,
            128,
        >,
    >,
    #[napi(skip)]
    pub call_other_contract_tsfn: Arc<
        ThreadsafeFunction<
            ThreadSafeJsImportResponse,
            Promise<Buffer>,
            ThreadSafeJsImportResponse,
            Status,
            true,
            false,
            128,
        >,
    >,
    #[napi(skip)]
    pub deploy_from_address_tsfn: Arc<
        ThreadsafeFunction<
            ThreadSafeJsImportResponse,
            Promise<Buffer>,
            ThreadSafeJsImportResponse,
            Status,
            true,
            false,
            128,
        >,
    >,
    #[napi(skip)]
    pub console_log_tsfn: Arc<
        ThreadsafeFunction<
            ThreadSafeJsImportResponse,
            Promise<()>,
            ThreadSafeJsImportResponse,
            Status,
            true,
            false,
            128,
        >,
    >,
    #[napi(skip)]
    pub emit_tsfn: Arc<
        ThreadsafeFunction<
            ThreadSafeJsImportResponse,
            Promise<()>,
            ThreadSafeJsImportResponse,
            Status,
            true,
            false,
            128,
        >,
    >,
    #[napi(skip)]
    pub inputs_tsfn: Arc<
        ThreadsafeFunction<
            ThreadSafeJsImportResponse,
            Promise<Buffer>,
            ThreadSafeJsImportResponse,
            Status,
            true,
            false,
            128,
        >,
    >,
    #[napi(skip)]
    pub outputs_tsfn: Arc<
        ThreadsafeFunction<
            ThreadSafeJsImportResponse,
            Promise<Buffer>,
            ThreadSafeJsImportResponse,
            Status,
            true,
            false,
            128,
        >,
    >,
    #[napi(skip)]
    pub account_type_tsfn: Arc<
        ThreadsafeFunction<
            ThreadSafeJsImportResponse,
            Promise<AccountTypeResponse>,
            ThreadSafeJsImportResponse,
            Status,
            true,
            false,
            128,
        >,
    >,
    #[napi(skip)]
    pub block_hash_tsfn: Arc<
        ThreadsafeFunction<
            BlockHashRequest,
            Promise<JsBlockHashResponse>,
            BlockHashRequest,
            Status,
            true,
            false,
            128,
        >,
    >,
}

#[napi] // noinspection RsCompileErrorMacro
impl ContractManager {
    #[napi(constructor)]
    pub fn new(
        max_idling_runtimes: u32,
        #[napi(
            ts_arg_type = "(err: Error, result: ThreadSafeJsImportResponse) => Promise<Buffer>"
        )]
        storage_load_js_function: Function<ThreadSafeJsImportResponse, Promise<Buffer>>,
        #[napi(
            ts_arg_type = "(err: Error, result: ThreadSafeJsImportResponse) => Promise<Buffer>"
        )]
        storage_store_js_function: Function<ThreadSafeJsImportResponse, Promise<Buffer>>,
        #[napi(
            ts_arg_type = "(err: Error, result: ThreadSafeJsImportResponse) => Promise<Buffer>"
        )]
        call_other_contract_js_function: Function<
            ThreadSafeJsImportResponse,
            Promise<Buffer>,
        >,
        #[napi(
            ts_arg_type = "(err: Error, result: ThreadSafeJsImportResponse) => Promise<Buffer>"
        )]
        deploy_from_address_js_function: Function<
            ThreadSafeJsImportResponse,
            Promise<Buffer>,
        >,
        #[napi(ts_arg_type = "(err: Error, result: ThreadSafeJsImportResponse) => Promise<void>")]
        console_log_js_function: Function<ThreadSafeJsImportResponse, Promise<()>>,
        #[napi(ts_arg_type = "(err: Error, result: ThreadSafeJsImportResponse) => Promise<void>")]
        emit_js_function: Function<ThreadSafeJsImportResponse, Promise<()>>,
        #[napi(
            ts_arg_type = "(err: Error, result: ThreadSafeJsImportResponse) => Promise<Buffer>"
        )]
        inputs_js_function: Function<ThreadSafeJsImportResponse, Promise<Buffer>>,
        #[napi(
            ts_arg_type = "(err: Error, result: ThreadSafeJsImportResponse) => Promise<Buffer>"
        )]
        outputs_js_function: Function<ThreadSafeJsImportResponse, Promise<Buffer>>,
        #[napi(
            ts_arg_type = "(err: Error, result: ThreadSafeJsImportResponse) => Promise<AccountTypeResponse>"
        )]
        account_type_js_function: Function<
            ThreadSafeJsImportResponse,
            Promise<AccountTypeResponse>,
        >,
        #[napi(
            ts_arg_type = "(err: Error, result: BlockHashRequest) => Promise<JsBlockHashResponse>"
        )]
        block_hash_js_function: Function<BlockHashRequest, Promise<JsBlockHashResponse>>,
    ) -> Result<Self, Error> {
        let storage_load_tsfn = build_tsfn!(
            storage_load_js_function,
            ThreadSafeJsImportResponse,
            Promise<Buffer>
        );

        let storage_store_tsfn = build_tsfn!(
            storage_store_js_function,
            ThreadSafeJsImportResponse,
            Promise<Buffer>
        );

        let call_other_contract_tsfn = build_tsfn!(
            call_other_contract_js_function,
            ThreadSafeJsImportResponse,
            Promise<Buffer>
        );

        let deploy_from_address_tsfn = build_tsfn!(
            deploy_from_address_js_function,
            ThreadSafeJsImportResponse,
            Promise<Buffer>
        );

        let console_log_tsfn = build_tsfn!(
            console_log_js_function,
            ThreadSafeJsImportResponse,
            Promise<()>
        );

        let emit_tsfn = build_tsfn!(emit_js_function, ThreadSafeJsImportResponse, Promise<()>);

        let inputs_tsfn = build_tsfn!(
            inputs_js_function,
            ThreadSafeJsImportResponse,
            Promise<Buffer>
        );

        let outputs_tsfn = build_tsfn!(
            outputs_js_function,
            ThreadSafeJsImportResponse,
            Promise<Buffer>
        );

        let account_type_tsfn = build_tsfn!(
            account_type_js_function,
            ThreadSafeJsImportResponse,
            Promise<AccountTypeResponse>
        );

        let block_hash_tsfn = build_tsfn!(
            block_hash_js_function,
            BlockHashRequest,
            Promise<JsBlockHashResponse>
        );

        let max_idling_runtimes = max_idling_runtimes as usize;
        let runtime_pool = Arc::new(RuntimePool::new(max_idling_runtimes));

        Ok(ContractManager {
            contracts: HashMap::new(),
            contract_cache: HashMap::new(),
            next_id: 1,
            storage_load_tsfn,
            storage_store_tsfn,
            call_other_contract_tsfn,
            deploy_from_address_tsfn,
            console_log_tsfn,
            runtime_pool,
            emit_tsfn,
            inputs_tsfn,
            outputs_tsfn,
            account_type_tsfn,
            block_hash_tsfn,
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
        bytecode: Option<BufferSlice>,
        used_gas: BigInt,
        max_gas: BigInt,
        memory_pages_used: BigInt,
        network: BitcoinNetworkRequest,
        is_debug_mode: bool,
        return_proofs: bool,
    ) -> Result<(), Error> {
        let used_gas = used_gas.get_u64().1;
        let max_gas = max_gas.get_u64().1;
        let id = reserved_id.get_u64().1;

        let mut params = JsContractParameter {
            bytecode: None,
            serialized: None,
            used_gas,
            max_gas,
            memory_pages_used: memory_pages_used.get_u64().1 as u32,
            network,
            is_debug_mode,
            return_proofs,
        };

        let mut should_cache = false;
        if let Some(serialized) = self.contract_cache.get(&address) {
            params.serialized = Some(serialized.clone());
        } else {
            let bc = bytecode
                .ok_or_else(|| Error::from_reason(anyhow!("Bytecode is required").to_string()))?
                .as_ref()
                .to_vec();

            should_cache = true;
            params.bytecode = Some(bc);
        }

        let js_contract = JsContract::from(params, self, id)?;
        if should_cache {
            let serialized = js_contract.serialize()?;
            self.contract_cache.insert(address, serialized);
        }

        // wrap in Arc
        let contract_arc = Arc::new(js_contract);
        self.add_contract(id, contract_arc)?;

        Ok(())
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
    pub fn destroy(&mut self, _env: Env) -> Result<(), Error> {
        /*abort_tsfn!(self.storage_load_tsfn, &env);
        abort_tsfn!(self.storage_store_tsfn, &env);
        abort_tsfn!(self.call_other_contract_tsfn, &env);
        abort_tsfn!(self.deploy_from_address_tsfn, &env);
        abort_tsfn!(self.console_log_tsfn, &env);
        abort_tsfn!(self.emit_tsfn, &env);
        abort_tsfn!(self.inputs_tsfn, &env);
        abort_tsfn!(self.outputs_tsfn, &env);
        abort_tsfn!(self.account_type_tsfn, &env);
        abort_tsfn!(self.block_hash_tsfn, &env);*/

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
    fn add_contract(&mut self, id: u64, contract: Arc<JsContract>) -> Result<u64, Error> {
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

    #[napi(ts_return_type = "ExitDataResponse")]
    pub fn get_exit_data(&self, env: Env, contract_id: BigInt) -> Result<Object, Error> {
        // 0️⃣ fetch contract
        let id = contract_id.get_u64().1;
        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;

        let mut exit_data = contract.get_exit_data()?;

        let mut js_object = Object::new(&env)?;
        js_object.set_named_property("status", exit_data.status)?;

        let js_buf = BufferSlice::copy_from(&env, std::mem::take(&mut exit_data.data))?;
        js_object.set_named_property("data", js_buf)?;
        js_object.set_named_property("gasUsed", exit_data.gas_used)?;

        let mut array = env.create_array(exit_data.proofs.len() as u32)?;
        for (idx, proof) in exit_data.proofs.iter().enumerate() {
            let proof_buffer = BufferSlice::copy_from(&env, proof.proof.clone())?;
            let vk_buffer = BufferSlice::copy_from(&env, proof.vk.clone())?;

            let mut obj = Object::new(&env)?;
            obj.set_named_property("proof", proof_buffer)?;
            obj.set_named_property("vk", vk_buffer)?;
            array.set_element(idx as u32, obj)?;
        }
        js_object.set_named_property("proofs", array)?;

        Ok(js_object)
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
    pub fn write_memory(&self, id: BigInt, offset: BigInt, data: Buffer) -> Result<(), Error> {
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

    #[napi]
    pub fn set_environment_variables(
        &self,
        id: BigInt,
        environment_variables: EnvironmentVariablesRequest,
    ) -> Result<(), Error> {
        let id = id.get_u64().1;

        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;

        contract.set_environment_variables(environment_variables)
    }

    #[napi(ts_return_type = "Promise<ExitDataResponse>")]
    pub fn on_deploy<'env>(
        &self,
        env: &'env Env,
        id: BigInt,
        calldata: BufferSlice,
    ) -> napi::Result<PromiseRaw<'env, ExitData>> {
        let id = id.get_u64().1;
        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason("Contract not found"))?
            .clone();

        let data = calldata.as_ref().to_vec();

        let fut = async move {
            let raw = tokio::task::spawn_blocking(move || contract.on_deploy(data))
                .await
                .map_err(|e| Error::from_reason(format!("Tokio join error: {e:?}")))??;

            Ok::<_, Error>(raw)
        };

        env.spawn_future(fut)
    }

    #[napi(ts_return_type = "Promise<ExitDataResponse>")]
    pub fn execute<'env>(
        &self,
        env: &'env Env,
        id: BigInt,
        calldata: BufferSlice,
    ) -> napi::Result<PromiseRaw<'env, ExitData>> {
        let id = id.get_u64().1;
        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason("Contract not found"))?
            .clone();

        let data = calldata.as_ref().to_vec();

        let fut = async move {
            let raw = tokio::task::spawn_blocking(move || contract.execute(data))
                .await
                .map_err(|e| Error::from_reason(format!("Tokio join error: {e:?}")))??;

            Ok::<_, Error>(raw)
        };

        env.spawn_future(fut)
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
