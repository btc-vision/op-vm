use crate::domain::runner::bitcoin_network::BitcoinNetwork;
use crate::domain::runner::{AbortData, InstanceWrapper};
use crate::interfaces::{
    CallOtherContractExternalFunction, ConsoleLogExternalFunction,
    DeployFromAddressExternalFunction, FlushEventsExternalFunction, StorageLoadExternalFunction,
    StorageStoreExternalFunction,
};
use std::sync::Arc;
use tokio::runtime::Runtime;

pub struct CustomEnv {
    pub instance: Option<InstanceWrapper>,
    pub network: BitcoinNetwork,
    pub abort_data: Option<AbortData>,
    pub storage_load_external: StorageLoadExternalFunction,
    pub storage_store_external: StorageStoreExternalFunction,
    pub call_other_contract_external: CallOtherContractExternalFunction,
    pub deploy_from_address_external: DeployFromAddressExternalFunction,
    pub console_log_external: ConsoleLogExternalFunction,
    pub flush_events_external: FlushEventsExternalFunction,
    pub runtime: Arc<Runtime>,
}

impl CustomEnv {
    pub fn new(
        network: BitcoinNetwork,
        storage_load_external: StorageLoadExternalFunction,
        storage_store_external: StorageStoreExternalFunction,
        call_other_contract_external: CallOtherContractExternalFunction,
        deploy_from_address_external: DeployFromAddressExternalFunction,
        console_log_external: ConsoleLogExternalFunction,
        flush_events_external: FlushEventsExternalFunction,
        runtime: Arc<Runtime>,
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
            flush_events_external,
            runtime,
        })
    }
}
