use crate::domain::common::Address;
use crate::domain::runner::EnvironmentVariables;

use napi::bindgen_prelude::BigInt;

#[cfg(feature = "use-strings-instead-of-buffers")]
use crate::domain::vm::hex_to_vec;

#[cfg(not(feature = "use-strings-instead-of-buffers"))]
use napi::bindgen_prelude::Buffer;

#[cfg(not(feature = "use-strings-instead-of-buffers"))]
#[napi(object)]
pub struct EnvironmentVariablesRequest {
    pub block_hash: Buffer,
    pub block_number: BigInt,
    pub block_median_time: BigInt,
    pub tx_id: Buffer,
    pub tx_hash: Buffer,
    pub contract_address: Buffer,
    pub contract_deployer: Buffer,
    pub caller: Buffer,
    pub origin: Buffer,
}

#[cfg(feature = "use-strings-instead-of-buffers")]
#[napi(object)]
pub struct EnvironmentVariablesRequest {
    pub block_hash: String,
    pub block_number: BigInt,
    pub block_median_time: BigInt,
    pub tx_id: String,
    pub tx_hash: String,
    pub contract_address: String,
    pub contract_deployer: String,
    pub caller: String,
    pub origin: String,
}

impl Into<EnvironmentVariables> for EnvironmentVariablesRequest {
    fn into(self) -> EnvironmentVariables {
        #[cfg(not(feature = "use-strings-instead-of-buffers"))]
        {
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
            )
        }

        #[cfg(feature = "use-strings-instead-of-buffers")]
        {
            let block_hash = hex_to_vec(self.block_hash).expect("block_hash");

            let tx_id = hex_to_vec(self.tx_id).expect("tx_id");
            let tx_hash = hex_to_vec(self.tx_hash).expect("tx_hash");
            let contract_address = hex_to_vec(self.contract_address).expect("contract_address");
            let contract_deployer = hex_to_vec(self.contract_deployer).expect("contract_deployer");
            let caller = hex_to_vec(self.caller).expect("caller");
            let origin = hex_to_vec(self.origin).expect("origin");

            EnvironmentVariables::new(
                &block_hash,
                self.block_number.get_u64().1,
                self.block_median_time.get_u64().1,
                &tx_id,
                &tx_hash,
                Address::new(&contract_address),
                Address::new(&contract_deployer),
                Address::new(&caller),
                Address::new(&origin),
            )
        }
    }
}
