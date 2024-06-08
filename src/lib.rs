#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::fs;

use napi::bindgen_prelude::Buffer;
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

#[napi]
pub fn call_test_1(function: String) -> i32 {
    let contract_bytecode = fs::read(CONTRACT_PATH).expect("Unable to read contract file");
    let mut contract = Contract::new(contract_bytecode, "bcrt1py2dhdrrf4s72gau3mkdw0mpnkgzp63qfdc0j7nah3luhmfcwf8kq4r44ef", "bcrt1pqdekymf30t583r8r9q95jyrgvyxcgrprajmyc9q8twae7ec275kq85vsev");
    let _ = contract.init();
    let response = contract.call(&function, &[]);
    let boxed = response.unwrap();
    let value: Value = boxed[0].clone();
    value.unwrap_i32()
}

#[napi]
pub fn call_test_2(function: String, data: Buffer) {
    let contract_bytecode = fs::read(CONTRACT_PATH).expect("Unable to read contract file");
    let mut contract = Contract::new(contract_bytecode, "bcrt1py2dhdrrf4s72gau3mkdw0mpnkgzp63qfdc0j7nah3luhmfcwf8kq4r44ef", "bcrt1pqdekymf30t583r8r9q95jyrgvyxcgrprajmyc9q8twae7ec275kq85vsev");
    let _ = contract.init();

    let bytes: Vec<u8> = data.into();
    let chunks = bytes.chunks(16);
    let raw_values: Vec<RawValue> = chunks.map(|chunk| RawValue { bytes: <[u8; 16]>::try_from(chunk).unwrap() }).collect();

    let _ = contract.call_raw(&function, raw_values);
}
