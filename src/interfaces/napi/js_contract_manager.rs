use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// For alpha.27, we'll do NapiResult alias or just use `napi::Result`
use crate::domain::tcp::{SocketConnection, SocketPool};
use crate::interfaces::napi::{
    bitcoin_network_request::BitcoinNetworkRequest, contract::JsContractParameter,
    js_contract::JsContract, runtime_pool::RuntimePool,
};
use crate::interfaces::{AbortDataResponse, ContractCallTask};
use anyhow::anyhow;
use bytes::Bytes;
use napi::bindgen_prelude::Buffer;
use napi::{
    bindgen_prelude::{AsyncTask, BigInt, Undefined},
    Env, Error, JsNumber, JsObject, Result as NapiResult,
};
use napi_derive::napi;
use tokio::runtime::Runtime;
use wasmer::{RuntimeError, Value};

#[napi(js_name = "ContractManager")]
pub struct ContractManager {
    contracts: HashMap<u64, JsContract>,
    contract_cache: HashMap<String, Bytes>,
    next_id: u64,

    runtime_pool: Arc<RuntimePool>,
    socket_pool: Arc<SocketPool>,
}

#[napi]
impl ContractManager {
    #[napi(constructor)]
    pub fn new(
        max_idling_runtimes: u32,
        socket_port: u16,
        max_sockets: u32,
    ) -> Result<Self, Error> {
        let max_idling_runtimes = max_idling_runtimes as usize;
        let max_sockets = max_sockets as usize;
        let runtime_pool = Arc::new(RuntimePool::new(max_idling_runtimes));
        let socket_pool = Arc::new(SocketPool::new(socket_port, max_sockets)?);

        Ok(Self {
            contracts: HashMap::new(),
            contract_cache: HashMap::new(),
            next_id: 1,
            runtime_pool,
            socket_pool,
        })
    }

    pub fn return_runtime(&self, runtime: Arc<Runtime>) -> anyhow::Result<()> {
        self.runtime_pool.return_runtime(runtime)
    }

    pub fn return_connection(
        &self,
        connection: Arc<Mutex<SocketConnection>>,
    ) -> Result<(), RuntimeError> {
        self.socket_pool.return_connection(connection)
    }

    pub fn get_connection(&self) -> Result<Arc<Mutex<SocketConnection>>, RuntimeError> {
        self.socket_pool.get_connection()
    }

    pub fn get_runtime(&self) -> Option<Arc<Runtime>> {
        self.runtime_pool.get_runtime()
    }

    pub fn get_runtime_pool(&self) -> Arc<RuntimePool> {
        self.runtime_pool.clone()
    }

    pub fn get_socket_pool(&self) -> Arc<SocketPool> {
        self.socket_pool.clone()
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

        println!("ContractManager::instantiate() done");
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

    #[napi]
    pub fn log(
        &self,
        env: Env,
        id: BigInt,
        func_name: String,
        params: Vec<JsNumber>,
    ) -> Result<(), Error> {
        let id = id.get_u64().1;

        println!(
            "--  ContractManager::call() calling contract function: {}, id: {}",
            func_name, id
        );

        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found (call)").to_string()))?;

        for p in params {
            let val = p
                .get_int32()
                .map_err(|e| Error::from_reason(format!("Invalid param: {:?}", e)))?;
            println!("--  ContractManager::call() param: {}", val);
        }

        Ok(())
    }

    #[napi(ts_return_type = "Promise<CallResponse>")]
    pub fn call(
        &self,
        env: Env,
        id: BigInt,
        func_name: String,
        params: Vec<JsNumber>,
    ) -> Result<JsObject, Error> {
        println!("--  ContractManager::call()");
        let id = id.get_u64().1;

        let contract = self
            .contracts
            .get(&id)
            .ok_or_else(|| Error::from_reason(anyhow!("Contract not found (call)").to_string()))?;

        println!(
            "--  ContractManager::call() calling contract function: {}, id: {}",
            func_name, id
        );

        let result = contract.call(env, func_name, params)?;
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
