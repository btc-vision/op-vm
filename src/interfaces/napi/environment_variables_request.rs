use crate::domain::common::Address;
use crate::domain::runner::EnvironmentVariables;
use napi::bindgen_prelude::{BigInt, Uint8Array};

#[napi(object)]
pub struct EnvironmentVariablesRequest {
    pub block_hash: Uint8Array,
    pub block_number: BigInt,
    pub block_median_time: BigInt,
    pub tx_id: Uint8Array,
    pub tx_hash: Uint8Array,
    pub contract_address: Uint8Array,
    pub contract_deployer: Uint8Array,
    pub caller: Uint8Array,
    pub origin: Uint8Array,
    pub chain_id: Uint8Array,
    pub protocol_id: Uint8Array,
}

impl Into<EnvironmentVariables> for EnvironmentVariablesRequest {
    fn into(self) -> EnvironmentVariables {
        EnvironmentVariables::new(
            &self.block_hash.to_vec(),
            self.block_number.get_u64().1,
            self.block_median_time.get_u64().1,
            &self.tx_id.to_vec(),
            &self.tx_hash.to_vec(),
            Address::new(&self.contract_address.to_vec()),
            Address::new(&self.contract_deployer.to_vec()),
            Address::new(&self.caller.to_vec()),
            Address::new(&self.origin.to_vec()),
            &self.chain_id,
            &self.protocol_id,
        )
    }
}
