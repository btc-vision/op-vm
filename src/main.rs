use std::fs;

use contract::Contract;

mod contract;
mod vm;

const CONTRACT_PATH: &str = "resources/release.wasm";
const CONTRACT_ADDRESS: &str = "bcrt1py2dhdrrf4s72gau3mkdw0mpnkgzp63qfdc0j7nah3luhmfcwf8kq4r44ef";
const DEPLOYER_ADDRESS: &str = "bcrt1pqdekymf30t583r8r9q95jyrgvyxcgrprajmyc9q8twae7ec275kq85vsev";

fn main() {
    let contract_bytecode = fs::read(CONTRACT_PATH).expect("Unable to read contract file");
    let mut contract = Contract::new(contract_bytecode, CONTRACT_ADDRESS, DEPLOYER_ADDRESS);
    contract.init();
    contract.call("getContract", &[]);
}
