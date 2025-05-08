use crate::domain::runner;
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
use napi::bindgen_prelude::BufferSlice;
use napi::bindgen_prelude::{BigInt, Buffer, Function, Promise};
use napi::threadsafe_function::ThreadsafeFunction;
use napi::Env;
use napi::{Error, JsNumber};
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

/*macro_rules! abort_tsfn {
    ($id:expr, $env:expr) => {
        if !$id.aborted() {
            $id.clone().abort()?;
        }

        $id.unref(&$env)
            .map_err(|e| Error::from_reason(format!("{:?}", e)))?;
    };
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
        //#[napi(
        //    ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer | Uint8Array>"
        //)]
        storage_load_js_function: Function<ThreadSafeJsImportResponse, Promise<Buffer>>,
        //#[napi(
        //    ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer | Uint8Array>"
        //)]
        storage_store_js_function: Function<ThreadSafeJsImportResponse, Promise<Buffer>>,
        //#[napi(
        //    ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer | Uint8Array>"
        //)]
        call_other_contract_js_function: Function<ThreadSafeJsImportResponse, Promise<Buffer>>,
        //#[napi(
        //    ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer | Uint8Array>"
        //)]
        deploy_from_address_js_function: Function<ThreadSafeJsImportResponse, Promise<Buffer>>,
        //#[napi(ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<void>")]
        console_log_js_function: Function<ThreadSafeJsImportResponse, Promise<()>>,
        //#[napi(ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<void>")]
        emit_js_function: Function<ThreadSafeJsImportResponse, Promise<()>>,
        //#[napi(
        //    ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer | Uint8Array>"
        //)]
        inputs_js_function: Function<ThreadSafeJsImportResponse, Promise<Buffer>>,
        //#[napi(
        //    ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer | Uint8Array>"
        //)]
        outputs_js_function: Function<ThreadSafeJsImportResponse, Promise<Buffer>>,
        //#[napi(
        //    ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<AccountTypeResponse>"
        //)]
        account_type_js_function: Function<
            ThreadSafeJsImportResponse,
            Promise<AccountTypeResponse>,
        >,
        //#[napi(
        //    ts_arg_type = "(_: never, result: BlockHashRequest) => Promise<JsBlockHashResponse>"
        //)]
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
        bytecode: Option<Buffer>,
        used_gas: BigInt,
        max_gas: BigInt,
        memory_pages_used: BigInt,
        network: BitcoinNetworkRequest,
        is_debug_mode: bool,
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
    pub fn get_exit_data(&self, env: Env, contract_id: BigInt) -> Result<napi::JsObject, Error> {
        let id = contract_id.get_u64().1;

        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;

        let exit_data = contract.get_exit_data()?;

        let mut js_object = env.create_object()?;
        js_object.set_named_property("status", env.create_uint32(exit_data.status))?;
        js_object.set_named_property(
            "data",
            BufferSlice::from_data(&env, exit_data.data.to_vec())?,
        )?;
        js_object.set_named_property("gasUsed", exit_data.gas_used)?;
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
    pub fn on_deploy(
        &self,
        env: Env,
        id: BigInt,
        calldata: Buffer,
    ) -> napi::Result<napi::JsObject> {
        let id_u64 = id.get_u64().1;
        let contract_arc = self
            .contracts
            .get(&id_u64)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?
            .clone();

        // We must clone the Arc for background usage and for final JS creation:
        let arc_for_bg = contract_arc.clone();

        let calldata_for_bg = calldata.to_vec();

        // The future to run in the background:
        let future = async move {
            // Inside spawn_blocking to avoid blocking async runtime
            let exit_data = tokio::task::spawn_blocking(move || {
                // The heavy-lifting synchronous call
                arc_for_bg.on_deploy(calldata_for_bg)
            })
            .await
            .map_err(|join_err| {
                Error::from_reason(format!("Tokio join error: {:?}", join_err))
            })??;

            // Return the result to the next closure
            Ok(exit_data)
        };

        // Now convert that `future` into a JS Promise using `execute_tokio_future`.
        let promise = env.execute_tokio_future(
            future,
            // This closure is run on the main thread to convert Rust data to JS objects
            move |&mut env, exit_data| {
                // use the second Arc to build a JS array
                let mut js_object = env.create_object()?;
                js_object.set_named_property("status", env.create_uint32(exit_data.status))?;
                js_object.set_named_property(
                    "data",
                    BufferSlice::from_data(&env, exit_data.data.to_vec())?,
                )?;
                js_object.set_named_property(
                    "gasUsed",
                    env.create_bigint_from_u64(exit_data.gas_used),
                )?;

                Ok(js_object)
            },
        )?;

        Ok(promise)
    }

    #[napi(ts_return_type = "Promise<ExitDataResponse>")]
    pub fn execute(&self, env: Env, id: BigInt, calldata: Buffer) -> napi::Result<napi::JsObject> {
        let id_u64 = id.get_u64().1;
        let contract_arc = self
            .contracts
            .get(&id_u64)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?
            .clone();

        // We must clone the Arc for background usage and for final JS creation:
        let arc_for_bg = contract_arc.clone();

        let calldata_for_bg = calldata.to_vec();

        // The future to run in the background:
        let future = async move {
            // Inside spawn_blocking to avoid blocking async runtime
            let exit_data = tokio::task::spawn_blocking(move || {
                // The heavy-lifting synchronous call
                arc_for_bg.execute(calldata_for_bg)
            })
            .await
            .map_err(|join_err| {
                Error::from_reason(format!("Tokio join error: {:?}", join_err))
            })??;

            // Return the result to the next closure
            Ok(exit_data)
        };

        // Now convert that `future` into a JS Promise using `execute_tokio_future`.
        let promise = env.execute_tokio_future(
            future,
            // This closure is run on the main thread to convert Rust data to JS objects
            move |&mut env, exit_data| {
                // use the second Arc to build a JS array
                let mut js_object = env.create_object()?;
                js_object.set_named_property("status", env.create_uint32(exit_data.status))?;
                js_object.set_named_property(
                    "data",
                    BufferSlice::from_data(&env, exit_data.data.to_vec())?,
                )?;
                js_object.set_named_property(
                    "gasUsed",
                    env.create_bigint_from_u64(exit_data.gas_used),
                )?;
                Ok(js_object)
            },
        )?;

        Ok(promise)
    }

    #[napi(ts_return_type = "Promise<number[]>")]
    pub fn call_export_by_name(
        &self,
        env: Env,
        id: BigInt,
        function_name: String,
        params: Vec<JsNumber>,
    ) -> napi::Result<napi::JsObject> {
        let id_u64 = id.get_u64().1;
        let contract_arc = self
            .contracts
            .get(&id_u64)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?
            .clone();

        // Convert JS numbers to i32
        let int_params: Vec<i32> = params
            .into_iter()
            .map(|num| num.get_int32())
            .collect::<napi::Result<Vec<i32>>>()?;

        // We must clone the Arc for background usage and for final JS creation:
        let arc_for_bg = contract_arc.clone();
        let arc_for_js = contract_arc.clone();

        let function_name_for_bg = function_name.clone();

        // The future to run in the background:
        let future = async move {
            // Inside spawn_blocking to avoid blocking async runtime
            let values_boxed = tokio::task::spawn_blocking(move || {
                // The heavy-lifting synchronous call
                arc_for_bg.call_export_by_name(&function_name_for_bg, &int_params)
            })
            .await
            .map_err(|join_err| {
                Error::from_reason(format!("Tokio join error: {:?}", join_err))
            })??;

            // Return the raw values to the next closure
            Ok(values_boxed)
        };

        // Now convert that `future` into a JS Promise using `execute_tokio_future`.
        let promise = env.execute_tokio_future(
            future,
            // This closure is run on the main thread to convert Rust data to JS objects
            move |&mut env, values_boxed| {
                // use the second Arc to build a JS array
                arc_for_js.convert_values_to_js_array(&env, values_boxed)
            },
        )?;

        Ok(promise)
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
