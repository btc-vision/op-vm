#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::fs;

use contract::Contract;

mod contract;
mod vm;

const CONTRACT_PATH: &str = "resources/release.wasm";

#[napi]
pub fn test(contract_address: String, deployer_address: String) {
    let contract_bytecode = fs::read(CONTRACT_PATH).expect("Unable to read contract file");
    let mut contract = Contract::new(contract_bytecode, &contract_address, &deployer_address);
    contract.init();
    contract.call("getContract", &[]);
}
