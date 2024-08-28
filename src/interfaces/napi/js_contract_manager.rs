use std::collections::HashMap;
use std::panic::catch_unwind;

use anyhow::anyhow;
use napi::{Env, Error, JsFunction, JsNumber};
use napi::bindgen_prelude::{AsyncTask, BigInt, Buffer, Undefined};

use crate::interfaces::{AbortDataResponse, ContractCallTask};
use crate::interfaces::napi::bitcoin_network_request::BitcoinNetworkRequest;
use crate::interfaces::napi::js_contract::JsContract;

#[napi(js_name = "ContractManager")]
pub struct ContractManager {
    contracts: HashMap<u64, JsContract>,
    next_id: u64,
}

#[napi]
impl ContractManager {
    #[napi(constructor)]
    pub fn new() -> Self {
        ContractManager {
            contracts: HashMap::new(),
            next_id: 1, // Start the ID counter at 1 (or 0, if preferred)
        }
    }

    #[napi]
    pub fn instantiate(&mut self, bytecode: Buffer,
                       max_gas: BigInt, network: BitcoinNetworkRequest, #[napi(
            ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer>"
        )]
                       storage_load_js_function: JsFunction,
                       #[napi(
                           ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer>"
                       )]
                       storage_store_js_function: JsFunction,
                       #[napi(
                           ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer>"
                       )]
                       call_other_contract_js_function: JsFunction,
                       #[napi(
                           ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<Buffer>"
                       )]
                       deploy_from_address_js_function: JsFunction,
                       #[napi(
                           ts_arg_type = "(_: never, result: ThreadSafeJsImportResponse) => Promise<void>"
                       )]
                       console_log_js_function: JsFunction) -> Result<BigInt, Error> {
        //catch_unwind(|| {
        let js_contract = JsContract::new(
            bytecode,
            max_gas,
            network,
            storage_load_js_function,
            storage_store_js_function,
            call_other_contract_js_function,
            deploy_from_address_js_function,
            console_log_js_function,
        )?;

        let id = self.add_contract(js_contract)?;

        Ok(BigInt::from(id))
        //})
        //     .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    #[napi]
    pub fn validate_bytecode(&self, bytecode: Buffer, max_gas: BigInt) -> Result<bool, Error> {
        JsContract::validate_bytecode(bytecode, max_gas)
    }

    #[napi]
    pub fn destroy(&mut self, env: Env, id: BigInt) -> Result<bool, Error> {
        //catch_unwind(|| {
        let id = id.get_u64().1;

        let contract = self.contracts.get_mut(&id).ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.destroy(env)?;

        match self.contracts.remove(&id) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
        //})
        //   .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    #[napi]
    pub fn destroy_all(&mut self, env: Env) -> Result<(), Error> {
        //catch_unwind(|| {
        for contract in self.contracts.values_mut() {
            contract.destroy(env)?;
        }

        self.contracts.clear();

        Ok(())
        //})
        //    .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    // Add a JsContract to the map and return its ID
    fn add_contract(&mut self, contract: JsContract) -> Result<u64, Error> {
        //catch_unwind(|| {
        if self.next_id > u64::MAX - 1 {
            //return Err(Error::from_reason(anyhow!("Maximum number of contracts reached").to_string()));
            self.next_id = 1;
        }

        let id = self.next_id;
        self.next_id += 1;
        self.contracts.insert(id, contract);

        Ok(id)
        //})
        //    .unwrap_or_else(|e| Err(Error::from_reason(format!("{:?}", e))))
    }

    #[napi]
    pub fn use_gas(&self, contract_id: BigInt, gas: BigInt) -> Result<(), Error> {
        let id = contract_id.get_u64().1;

        let contract = self.contracts.get(&id).ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.use_gas(gas)
    }

    #[napi]
    pub fn write_buffer(&self, contract_id: BigInt, value: Buffer, id: i32, align: u32) -> Result<i64, Error> {
        let contract_id = contract_id.get_u64().1;

        let contract = self.contracts.get(&contract_id).ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.write_buffer(value, id, align)
    }

    #[napi]
    pub fn get_abort_data(&self, contract_id: BigInt) -> Result<AbortDataResponse, Error> {
        let id = contract_id.get_u64().1;

        let contract = self.contracts.get(&id).ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.get_abort_data()
    }


    #[napi]
    pub fn set_remaining_gas(&self, id: BigInt, gas: BigInt) -> Result<(), Error> {
        let id = id.get_u64().1;

        let contract = self.contracts.get(&id).ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.set_remaining_gas(gas)
    }

    #[napi]
    pub fn get_remaining_gas(&self, id: BigInt) -> Result<BigInt, Error> {
        let id = id.get_u64().1;

        let contract = self.contracts.get(&id).ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.get_remaining_gas()
    }

    #[napi]
    pub fn set_used_gas(&self, id: BigInt, gas: BigInt) -> Result<(), Error> {
        let id = id.get_u64().1;

        let contract = self.contracts.get(&id).ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.set_used_gas(gas)
    }

    #[napi]
    pub fn get_used_gas(&self, id: BigInt) -> Result<BigInt, Error> {
        let id = id.get_u64().1;

        let contract = self.contracts.get(&id).ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.get_used_gas()
    }

    #[napi]
    pub fn write_memory(&self, id: BigInt, offset: BigInt, data: Buffer) -> Result<Undefined, Error> {
        let id = id.get_u64().1;

        let contract = self.contracts.get(&id).ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        contract.write_memory(offset, data)
    }

    #[napi]
    pub fn read_memory(&self, id: BigInt, offset: BigInt, length: BigInt) -> Result<Buffer, Error> {
        let id = id.get_u64().1;

        let contract = self.contracts.get(&id).ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
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

        let contract = self.contracts.get(&id).ok_or_else(|| Error::from_reason(anyhow!("Contract not found").to_string()))?;
        let result = contract.call(func_name, params)?;

        Ok(result)
    }

    #[napi]
    pub fn length(&self) -> Result<BigInt, Error> {
        Ok(BigInt::from(self.contracts.len() as u64))
    }

    #[napi]
    pub fn clear(&mut self, env: Env) -> Result<(), Error> {
        for contract in self.contracts.values_mut() {
            contract.destroy(env)?;
        }

        self.contracts.clear();

        Ok(())
    }
}
