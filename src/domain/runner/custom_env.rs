use sha2::{Digest, Sha256};
use wasmer::RuntimeError;

use crate::domain::runner::{AbortData, InstanceWrapper};
use crate::domain::runner::bitcoin_network::BitcoinNetwork;
use crate::interfaces::{
    CallOtherContractExternalFunction, ConsoleLogExternalFunction,
    DeployFromAddressExternalFunction, EncodeAddressExternalFunction, StorageLoadExternalFunction,
    StorageStoreExternalFunction,
};

pub struct CustomEnv {
    pub instance: Option<InstanceWrapper>,
    pub network: BitcoinNetwork,
    pub abort_data: Option<AbortData>,
    pub storage_load_external: StorageLoadExternalFunction,
    pub storage_store_external: StorageStoreExternalFunction,
    pub call_other_contract_external: CallOtherContractExternalFunction,
    pub deploy_from_address_external: DeployFromAddressExternalFunction,
    pub console_log_external: ConsoleLogExternalFunction,
    pub encode_address_external: EncodeAddressExternalFunction,
}

impl CustomEnv {
    pub fn new(
        network: BitcoinNetwork,
        storage_load_external: StorageLoadExternalFunction,
        storage_store_external: StorageStoreExternalFunction,
        call_other_contract_external: CallOtherContractExternalFunction,
        deploy_from_address_external: DeployFromAddressExternalFunction,
        console_log_external: ConsoleLogExternalFunction,
        encode_address_external: EncodeAddressExternalFunction,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            instance: None,
            network,
            abort_data: None,
            storage_load_external,
            storage_store_external,
            call_other_contract_external,
            deploy_from_address_external,
            console_log_external,
            encode_address_external,
        })
    }

    pub fn sha256(&self, data: &[u8]) -> Result<Vec<u8>, RuntimeError> {
        let hash = Sha256::digest(data);
        let hash_as_vec: Vec<u8> = hash.to_vec();

        Ok(hash_as_vec)
    }
}
