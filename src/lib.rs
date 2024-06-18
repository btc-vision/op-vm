#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::fs;
use std::panic;

use crate::domain::contract::Contract;

mod domain;
mod interfaces;

const CONTRACT_PATH: &str = "resources/release.wasm";


#[napi]
pub fn init() {
    panic::set_hook(Box::new(|_| {}));
}

#[napi]
pub fn test(contract_address: String, deployer_address: String) {
    let contract_bytecode = fs::read(CONTRACT_PATH).expect("Unable to read contract file");
    let mut contract = Contract::new(&contract_bytecode);
    let _ = contract.init(&contract_address, &deployer_address);
    let _ = contract.call("getContract", &[]);
}


