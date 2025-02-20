use crate::domain::runner::bitcoin_network::BitcoinNetwork;
use crate::domain::runner::call_result::CallResult;
use crate::domain::runner::{AbortData, InstanceWrapper};
use crate::interfaces::{
    CallOtherContractExternalFunction, ConsoleLogExternalFunction,
    DeployFromAddressExternalFunction, EmitExternalFunction, InputsExternalFunction,
    OutputsExternalFunction, StorageLoadExternalFunction, StorageStoreExternalFunction,
};
use std::sync::{Arc, Mutex};
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
    pub emit_external: EmitExternalFunction,
    pub inputs_external: InputsExternalFunction,
    pub outputs_external: OutputsExternalFunction,
    pub runtime: Arc<Runtime>,
    pub store_cache: Mutex<super::store::Cache>,
    pub last_call_result: CallResult,
}

impl CustomEnv {
    pub fn new(
        network: BitcoinNetwork,
        storage_load_external: StorageLoadExternalFunction,
        storage_store_external: StorageStoreExternalFunction,
        call_other_contract_external: CallOtherContractExternalFunction,
        deploy_from_address_external: DeployFromAddressExternalFunction,
        console_log_external: ConsoleLogExternalFunction,
        emit_external: EmitExternalFunction,
        inputs_external: InputsExternalFunction,
        outputs_external: OutputsExternalFunction,
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
            emit_external,
            inputs_external,
            outputs_external,
            runtime,
            store_cache: Mutex::new(super::store::Cache::new()),
            last_call_result: CallResult::default(),
        })
    }
}
