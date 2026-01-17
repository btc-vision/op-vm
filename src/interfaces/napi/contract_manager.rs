use crate::domain::runner::ExitData;
use crate::interfaces::napi::bitcoin_network_request::BitcoinNetworkRequest;
use crate::interfaces::napi::contract::Contract;
use crate::interfaces::napi::contract::ContractParameter;
use crate::interfaces::napi::environment_variables_request::EnvironmentVariablesRequest;
use crate::interfaces::napi::runtime_pool::RuntimePool;
use bytes::Bytes;
use neon::prelude::*;
use neon::types::buffer::TypedArray;
use neon::types::JsBigInt;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::INNER;

pub struct ContractManager {
    contracts: HashMap<u64, Contract>,
    contract_cache: HashMap<String, Bytes>,
    next_id: u64,
    pub runtime_pool: Arc<RuntimePool>,
    pub storage_load_js_function: Arc<Root<JsFunction>>,
    pub storage_store_js_function: Arc<Root<JsFunction>>,
    pub call_other_contract_js_function: Arc<Root<JsFunction>>,
    pub deploy_from_address_js_function: Arc<Root<JsFunction>>,
    pub update_from_address_js_function: Arc<Root<JsFunction>>,
    pub console_log_js_function: Arc<Root<JsFunction>>,
    pub emit_js_function: Arc<Root<JsFunction>>,
    pub inputs_js_function: Arc<Root<JsFunction>>,
    pub outputs_js_function: Arc<Root<JsFunction>>,
    pub account_type_js_function: Arc<Root<JsFunction>>,
    pub block_hash_js_function: Arc<Root<JsFunction>>,
    pub mldsa_load_js_function: Arc<Root<JsFunction>>,
}

impl Finalize for ContractManager {}

// TODO: Replace for mutex?
type BoxedContractManager = JsBox<Arc<Mutex<ContractManager>>>;

/// JS wrapped functions
impl ContractManager {
    pub fn js_constructor<'a>(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let this: Handle<'_, JsObject> = cx.this::<JsObject>()?;

        let max_idling_runtimes = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;
        let storage_load_js_function = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let storage_store_js_function = cx.argument::<JsFunction>(2)?.root(&mut cx);
        let call_other_contract_js_function = cx.argument::<JsFunction>(3)?.root(&mut cx);
        let deploy_from_address_js_function = cx.argument::<JsFunction>(4)?.root(&mut cx);
        let update_from_address_js_function = cx.argument::<JsFunction>(5)?.root(&mut cx);
        let console_log_js_function = cx.argument::<JsFunction>(6)?.root(&mut cx);
        let emit_js_function = cx.argument::<JsFunction>(7)?.root(&mut cx);
        let inputs_js_function = cx.argument::<JsFunction>(8)?.root(&mut cx);
        let outputs_js_function = cx.argument::<JsFunction>(9)?.root(&mut cx);
        let account_type_js_function = cx.argument::<JsFunction>(10)?.root(&mut cx);
        let block_hash_js_function = cx.argument::<JsFunction>(11)?.root(&mut cx);
        let mldsa_load_js_function = cx.argument::<JsFunction>(12)?.root(&mut cx);

        let inner = cx.boxed(Arc::new(Mutex::new(ContractManager::new(
            max_idling_runtimes,
            storage_load_js_function,
            storage_store_js_function,
            call_other_contract_js_function,
            deploy_from_address_js_function,
            update_from_address_js_function,
            console_log_js_function,
            emit_js_function,
            inputs_js_function,
            outputs_js_function,
            account_type_js_function,
            block_hash_js_function,
            mldsa_load_js_function,
        )?)));

        this.set(&mut cx, INNER, inner)?;
        Ok(cx.undefined())
    }

    pub fn js_reserve_id(mut cx: FunctionContext) -> JsResult<JsBigInt> {
        let this = cx.this::<JsObject>()?;
        let inner = this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let mut manager = inner
            .lock()
            .or_else(|err| cx.throw_error(err.to_string()))?;
        let id = manager.increment_next_id();

        Ok(JsBigInt::from_u64(&mut cx, id))
    }

    pub fn js_instantiate(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let this = cx.this::<JsObject>()?;
        let inner = this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let mut manager = inner
            .lock()
            .or_else(|err| cx.throw_error(err.to_string()))?;

        let reserved_id = cx
            .argument::<JsBigInt>(0)?
            .to_u64(&mut cx)
            .or_else(|e| cx.throw_range_error(e.to_string()))?;

        let address = cx.argument::<JsString>(1)?.value(&mut cx);
        let bytecode = cx.argument::<JsValue>(2)?;
        let used_gas = cx
            .argument::<JsBigInt>(3)?
            .to_u64(&mut cx)
            .or_else(|e| cx.throw_range_error(e.to_string()))?;

        let max_gas = cx
            .argument::<JsBigInt>(4)?
            .to_u64(&mut cx)
            .or_else(|e| cx.throw_range_error(e.to_string()))?;

        let memory_pages_used =
            cx.argument::<JsBigInt>(5)?
                .to_u64(&mut cx)
                .or_else(|e| cx.throw_range_error(e.to_string()))? as u32;

        let network_number = cx.argument::<JsNumber>(6)?.value(&mut cx) as u8;
        let network =
            BitcoinNetworkRequest::try_from(network_number).or_else(|e| cx.throw_range_error(e))?;

        let is_debug_mode = cx.argument::<JsBoolean>(7)?.value(&mut cx);

        let mut params = ContractParameter {
            bytecode: None,
            serialized: None,
            used_gas,
            max_gas,
            memory_pages_used,
            network,
            is_debug_mode,
        };

        let mut should_cache = false;
        if let Some(serialized) = manager.contract_cache.get(&address) {
            params.serialized = Some(serialized.clone());
        } else {
            let bc = bytecode
                .downcast::<JsBuffer, _>(&mut cx)
                .or_else(|e| cx.throw_error(e.to_string()))?
                .as_slice(&mut cx)
                .to_vec();
            should_cache = true;
            params.bytecode = Some(bc);
        }

        let contract = Contract::from(&mut cx, params, &manager, reserved_id)?;
        if should_cache {
            let serialized = contract
                .serialize()
                .or_else(|err| cx.throw_error(err.to_string()))?;
            manager.contract_cache.insert(address, serialized);
        }

        manager.add_contract(reserved_id, contract);

        Ok(cx.undefined())
    }

    pub fn js_destroy_contract(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let this = cx.this::<JsObject>()?;
        let inner = this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let mut manager = inner
            .lock()
            .or_else(|err| cx.throw_error(err.to_string()))?;

        let contract_id = cx
            .argument::<JsBigInt>(0)?
            .to_u64(&mut cx)
            .or_else(|e| cx.throw_range_error(e.to_string()))?;

        Ok(cx.boolean(manager.destroy_contract(contract_id)))
    }

    pub fn js_destroy(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let this = cx.this::<JsObject>()?;
        let value = cx.undefined();
        this.set(&mut cx, INNER, value)?;

        Ok(cx.undefined())
    }

    pub fn js_destroy_cache(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let this = cx.this::<JsObject>()?;
        let inner = this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let mut manager = inner
            .lock()
            .or_else(|err| cx.throw_error(err.to_string()))?;

        manager.destroy_cache();
        Ok(cx.undefined())
    }

    pub fn js_destroy_all(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let this = cx.this::<JsObject>()?;
        let inner = this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let mut manager = inner
            .lock()
            .or_else(|err| cx.throw_error(err.to_string()))?;

        manager.destroy_all();
        Ok(cx.undefined())
    }

    pub fn js_use_gas(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let this = cx.this::<JsObject>()?;
        let inner = this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let manager = inner
            .lock()
            .or_else(|err| cx.throw_error(err.to_string()))?;

        let contract_id = cx
            .argument::<JsBigInt>(0)?
            .to_u64(&mut cx)
            .or_throw(&mut cx)?;
        let gas = cx
            .argument::<JsBigInt>(1)?
            .to_u64(&mut cx)
            .or_throw(&mut cx)?;

        manager
            .use_gas(contract_id, gas)
            .or_else(|err| cx.throw_error(err.to_string()))?;

        Ok(cx.undefined())
    }

    pub fn js_get_exit_data(mut cx: FunctionContext) -> JsResult<JsObject> {
        let this = cx.this::<JsObject>()?;
        let inner = this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let manager = inner
            .lock()
            .or_else(|err| cx.throw_error(err.to_string()))?;

        let contract_id = cx
            .argument::<JsBigInt>(0)?
            .to_u64(&mut cx)
            .or_throw(&mut cx)?;

        let data = manager
            .get_exit_data(contract_id)
            .or_else(|err| cx.throw_error(err.to_string()))?;

        Ok(data.to_js_object(&mut cx)?)
    }

    pub fn js_get_used_gas(mut cx: FunctionContext) -> JsResult<JsBigInt> {
        let this = cx.this::<JsObject>()?;
        let inner = this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let manager = inner
            .lock()
            .or_else(|err| cx.throw_error(err.to_string()))?;
        let contract_id = cx
            .argument::<JsBigInt>(0)?
            .to_u64(&mut cx)
            .or_throw(&mut cx)?;

        let gas = manager
            .get_used_gas(contract_id)
            .or_else(|err| cx.throw_error(err.to_string()))?;

        Ok(JsBigInt::from_u64(&mut cx, gas))
    }

    pub fn js_write_memory(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let this = cx.this::<JsObject>()?;
        let inner = this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let manager = inner
            .lock()
            .or_else(|err| cx.throw_error(err.to_string()))?;

        let contract_id = cx
            .argument::<JsBigInt>(0)?
            .to_u64(&mut cx)
            .or_throw(&mut cx)?;
        let offset = cx
            .argument::<JsBigInt>(1)?
            .to_u64(&mut cx)
            .or_throw(&mut cx)?;
        let data = cx.argument::<JsBuffer>(2)?.as_slice(&mut cx);

        manager
            .write_memory(contract_id, offset, data)
            .or_else(|err| cx.throw_error(err.to_string()))?;

        Ok(cx.undefined())
    }

    pub fn js_read_memory(mut cx: FunctionContext) -> JsResult<JsBuffer> {
        let this = cx.this::<JsObject>()?;
        let inner = this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let manager = inner
            .lock()
            .or_else(|err| cx.throw_error(err.to_string()))?;
        let contract_id = cx
            .argument::<JsBigInt>(0)?
            .to_u64(&mut cx)
            .or_throw(&mut cx)?;
        let offset = cx
            .argument::<JsBigInt>(1)?
            .to_u64(&mut cx)
            .or_throw(&mut cx)?;
        let length = cx
            .argument::<JsBigInt>(2)?
            .to_u64(&mut cx)
            .or_throw(&mut cx)?;

        let data = manager
            .read_memory(contract_id, offset, length)
            .or_else(|err| cx.throw_error(err.to_string()))?;

        Ok(JsBuffer::from_slice(&mut cx, &data)?)
    }

    pub fn js_set_environment_variables(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let this = cx.this::<JsObject>()?;
        let inner = this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let mut manager = inner
            .lock()
            .or_else(|err| cx.throw_error(err.to_string()))?;

        let contract_id = cx
            .argument::<JsBigInt>(0)?
            .to_u64(&mut cx)
            .or_throw(&mut cx)?;

        let evr_object = cx.argument::<JsObject>(1)?;
        let environment_variables =
            EnvironmentVariablesRequest::from_js_object(&mut cx, evr_object)?;

        manager
            .set_environment_variables(contract_id, environment_variables)
            .or_else(|err| cx.throw_error(err.to_string()))?;

        Ok(cx.undefined())
    }

    pub fn js_on_deploy(mut cx: FunctionContext) -> JsResult<JsPromise> {
        let this = cx.this::<JsObject>()?;
        let inner: Handle<'_, BoxedContractManager> =
            this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let inner = Arc::clone(&inner);

        let contract_id = cx
            .argument::<JsBigInt>(0)?
            .to_u64(&mut cx)
            .or_throw(&mut cx)?;
        let calldata = cx.argument::<JsBuffer>(1)?.as_slice(&mut cx).to_vec();
        let (deferred, promise) = cx.promise();
        let channel = cx.channel();

        // Call task on background
        let contract = inner
            .lock()
            .unwrap()
            .get_contract(contract_id)
            .unwrap()
            .clone();

        std::thread::spawn(move || {
            let result = contract.on_deploy(calldata);

            // Sync with main JS thread
            channel.send(move |mut cx| match result {
                Ok(exit_data) => {
                    let result = exit_data.to_js_object(&mut cx).unwrap();
                    Ok(deferred.resolve(&mut cx, result))
                }
                Err(err) => {
                    let error = cx.string(err.to_string());
                    deferred.reject(&mut cx, error);
                    Ok(())
                }
            })
        });

        Ok(promise)
    }

    pub fn js_on_update(mut cx: FunctionContext) -> JsResult<JsPromise> {
        let this = cx.this::<JsObject>()?;
        let inner: Handle<'_, BoxedContractManager> =
            this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let inner = Arc::clone(&inner);

        let contract_id = cx
            .argument::<JsBigInt>(0)?
            .to_u64(&mut cx)
            .or_throw(&mut cx)?;
        let calldata = cx.argument::<JsBuffer>(1)?.as_slice(&mut cx).to_vec();
        let (deferred, promise) = cx.promise();
        let channel = cx.channel();

        // Call task on background
        let contract = inner
            .lock()
            .unwrap()
            .get_contract(contract_id)
            .unwrap()
            .clone();

        std::thread::spawn(move || {
            let result = contract.on_update(calldata);

            // Sync with main JS thread
            channel.send(move |mut cx| match result {
                Ok(exit_data) => {
                    let result = exit_data.to_js_object(&mut cx).unwrap();
                    Ok(deferred.resolve(&mut cx, result))
                }
                Err(err) => {
                    let error = cx.string(err.to_string());
                    deferred.reject(&mut cx, error);
                    Ok(())
                }
            })
        });

        Ok(promise)
    }

    pub fn js_execute(mut cx: FunctionContext) -> JsResult<JsPromise> {
        let this = cx.this::<JsObject>()?;
        let inner: Handle<'_, BoxedContractManager> =
            this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let inner = Arc::clone(&inner);

        let contract_id = cx
            .argument::<JsBigInt>(0)?
            .to_u64(&mut cx)
            .or_throw(&mut cx)?;
        let calldata = cx.argument::<JsBuffer>(1)?.as_slice(&mut cx).to_vec();
        let (deferred, promise) = cx.promise();
        let channel = cx.channel();
        let contract = inner
            .lock()
            .unwrap()
            .get_contract(contract_id)
            .unwrap()
            .clone();

        std::thread::spawn(move || {
            let result = contract.execute(calldata);

            // Sync with main JS thread
            channel.send(move |mut cx| match result {
                Ok(exit_data) => {
                    let result = exit_data.to_js_object(&mut cx).unwrap();
                    Ok(deferred.resolve(&mut cx, result))
                }
                Err(err) => {
                    let string = err.to_string();
                    let error = cx.error(string.clone())?;

                    deferred.reject(&mut cx, error);
                    Ok(())
                }
            })
        });

        Ok(promise)
    }

    pub fn js_length(mut cx: FunctionContext) -> JsResult<JsBigInt> {
        let this = cx.this::<JsObject>()?;
        let inner = this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let manager = inner
            .lock()
            .or_else(|err| cx.throw_error(err.to_string()))?;

        let length = manager
            .length()
            .or_else(|err| cx.throw_error(err.to_string()))?;

        Ok(JsBigInt::from_u64(&mut cx, length))
    }

    pub fn js_clear(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let this = cx.this::<JsObject>()?;
        let inner = this.get::<BoxedContractManager, _, _>(&mut cx, INNER)?;
        let mut manager = inner
            .lock()
            .or_else(|err| cx.throw_error(err.to_string()))?;

        manager
            .clear()
            .or_else(|err| cx.throw_error(err.to_string()))?;

        Ok(cx.undefined())
    }
}

impl ContractManager {
    pub fn new(
        max_idling_runtimes: u32,
        storage_load_js_function: Root<JsFunction>,
        storage_store_js_function: Root<JsFunction>,
        call_other_contract_js_function: Root<JsFunction>,
        deploy_from_address_js_function: Root<JsFunction>,
        update_from_address_js_function: Root<JsFunction>,
        console_log_js_function: Root<JsFunction>,
        emit_js_function: Root<JsFunction>,
        inputs_js_function: Root<JsFunction>,
        outputs_js_function: Root<JsFunction>,
        account_type_js_function: Root<JsFunction>,
        block_hash_js_function: Root<JsFunction>,
        mldsa_load_js_function: Root<JsFunction>,
    ) -> NeonResult<Self> {
        let max_idling_runtimes = max_idling_runtimes as usize;
        let runtime_pool = Arc::new(RuntimePool::new(max_idling_runtimes));

        Ok(ContractManager {
            contracts: HashMap::new(),
            contract_cache: HashMap::new(),
            next_id: 1,
            storage_load_js_function: Arc::new(storage_load_js_function),
            storage_store_js_function: Arc::new(storage_store_js_function),
            call_other_contract_js_function: Arc::new(call_other_contract_js_function),
            deploy_from_address_js_function: Arc::new(deploy_from_address_js_function),
            update_from_address_js_function: Arc::new(update_from_address_js_function),
            console_log_js_function: Arc::new(console_log_js_function),
            runtime_pool,
            emit_js_function: Arc::new(emit_js_function),
            inputs_js_function: Arc::new(inputs_js_function),
            outputs_js_function: Arc::new(outputs_js_function),
            account_type_js_function: Arc::new(account_type_js_function),
            block_hash_js_function: Arc::new(block_hash_js_function),
            mldsa_load_js_function: Arc::new(mldsa_load_js_function),
        })
    }

    pub fn destroy_contract(&mut self, id: u64) -> bool {
        match self.contracts.remove(&id) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn destroy_cache(&mut self) -> () {
        self.contract_cache.clear();

        ()
    }

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
    fn add_contract(&mut self, id: u64, contract: Contract) -> u64 {
        self.contracts.insert(id, contract);
        id
    }

    pub fn get_contract(&self, id: u64) -> anyhow::Result<&Contract> {
        self.contracts
            .get(&id)
            .ok_or(anyhow::anyhow!("Contract not found"))
    }

    pub fn use_gas(&self, contract_id: u64, gas: u64) -> anyhow::Result<()> {
        self.get_contract(contract_id)?.use_gas(gas)
    }

    pub fn get_exit_data(&self, contract_id: u64) -> anyhow::Result<ExitData> {
        self.get_contract(contract_id)?.get_exit_data()
    }

    pub fn get_used_gas(&self, contract_id: u64) -> anyhow::Result<u64> {
        Ok(self.get_contract(contract_id)?.get_used_gas()?)
    }

    pub fn write_memory(&self, contract_id: u64, offset: u64, data: &[u8]) -> anyhow::Result<()> {
        self.get_contract(contract_id)?.write_memory(offset, data)
    }

    pub fn read_memory(
        &self,
        contract_id: u64,
        offset: u64,
        length: u64,
    ) -> anyhow::Result<Vec<u8>> {
        self.get_contract(contract_id)?.read_memory(offset, length)
    }

    pub fn set_environment_variables(
        &mut self,
        contract_id: u64,
        environment_variables: EnvironmentVariablesRequest,
    ) -> anyhow::Result<()> {
        self.contracts
            .get_mut(&contract_id)
            .unwrap()
            .set_environment_variables(environment_variables)?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn on_deploy(&self, contract_id: u64, calldata: Vec<u8>) -> anyhow::Result<ExitData> {
        self.get_contract(contract_id)?.on_deploy(calldata)
    }

    #[allow(dead_code)]
    pub fn on_update(&self, contract_id: u64, calldata: Vec<u8>) -> anyhow::Result<ExitData> {
        self.get_contract(contract_id)?.on_update(calldata)
    }

    #[allow(dead_code)]
    pub fn execute(&self, contract_id: u64, calldata: Vec<u8>) -> anyhow::Result<ExitData> {
        let result = self.get_contract(contract_id)?.execute(calldata);
        result
    }

    pub fn length(&self) -> anyhow::Result<u64> {
        Ok(self.contracts.len() as u64)
    }

    pub fn clear(&mut self) -> anyhow::Result<()> {
        //for contract in self.contracts.values_mut() {
        //    contract.destroy(env)?;
        //}

        self.contracts.clear();

        Ok(())
    }
}
