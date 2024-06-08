#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::fs;

use napi::bindgen_prelude::{BigInt, Buffer};
use wasmer::Value;
use wasmer_types::RawValue;

use contract::Contract;

mod contract;
mod vm;

const CONTRACT_PATH: &str = "resources/release.wasm";

#[napi]
pub fn test(contract_address: String, deployer_address: String) {
    let contract_bytecode = fs::read(CONTRACT_PATH).expect("Unable to read contract file");
    let mut contract = Contract::new(contract_bytecode, &contract_address, &deployer_address);
    let _ = contract.init();
    let _ = contract.call("getContract", &[]);
}


#[napi(js_name = "ContractAPI")]
struct ContractAPI {
    contract: Contract,
}

#[napi] //noinspection RsCompileErrorMacro
impl ContractAPI {
    #[napi(constructor)]
    pub fn new(contract_address: String, deployer_address: String) -> Self {
        let contract_bytecode = fs::read(CONTRACT_PATH).expect("Unable to read contract file");
        let contract = Contract::new(contract_bytecode, &contract_address, &deployer_address);
        Self { contract }
    }

    #[napi]
    pub fn init(&mut self) {
        self.contract.init().unwrap();
    }

    #[napi(js_name = "__new")]
    pub fn __new(&mut self, size: i32, id: i32) -> i32 {
        let __new = self.contract.instance.exports.get_function("__new").unwrap();
        let response = __new.call(&mut self.contract.store, &[Value::I32(size), Value::I32(id)]);
        let boxed = response.unwrap();
        let value: Value = boxed[0].clone();
        value.unwrap_i32()
    }

    #[napi(js_name = "__pin")]
    pub fn __pin(&mut self, ptr: i32) -> i32 {
        let __new = self.contract.instance.exports.get_function("__new").unwrap();
        let response = __new.call(&mut self.contract.store, &[Value::I32(ptr)]);
        let boxed = response.unwrap();
        let value: Value = boxed[0].clone();
        value.unwrap_i32()
    }

    #[napi]
    pub fn call_test_1(&mut self, func_name: String) -> i32 {
        let response = self.contract.call(&func_name, &[]);
        let boxed = response.unwrap();
        let value: Value = boxed[0].clone();
        value.unwrap_i32()
    }

    #[napi]
    pub fn call_test_2(&mut self, func_name: String, data: Buffer) {
        let bytes: Vec<u8> = data.into();
        let chunks = bytes.chunks(16);
        // let raw_values: Vec<RawValue> = chunks
        //     .map(|chunk| RawValue {
        //         bytes: <[u8; 16]>::try_from(chunk),
        //     })
        //     .collect();

        let mut raw_values: Vec<RawValue> = Vec::new();
        for chunk in chunks {
            if chunk.len() < 16 {
                let mut padded_chunk = vec![0; 16];
                padded_chunk[..chunk.len()].copy_from_slice(chunk);
                raw_values.push(RawValue {
                    bytes: padded_chunk.try_into().unwrap(),
                });
            } else {
                raw_values.push(RawValue {
                    bytes: chunk.try_into().unwrap(),
                });
            }
        }

        self.contract.call_raw(&func_name, raw_values).unwrap();
    }

    #[napi]
    pub fn read_memory(&self, offset: BigInt, length: BigInt) -> Buffer {
        let memory = self.contract.instance.exports.get_memory("memory").unwrap();
        let mut buf: Vec<u8> = vec![0; length.get_u64().1 as usize];
        memory.view(&self.contract.store).read(offset.get_u64().1, &mut buf).unwrap();
        Buffer::from(buf)
    }

    #[napi]
    pub fn write_memory(&self, offset: BigInt, data: Buffer) {
        let data: Vec<u8> = data.into();
        let memory = self.contract.instance.exports.get_memory("memory").unwrap();
        memory.view(&self.contract.store).write(offset.get_u64().1, &data).unwrap();
    }
}
