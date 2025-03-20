use crate::domain::runner::environment_variables::EnvironmentVariables;
use crate::domain::runner::{
    BitcoinNetwork, Cache, CallResult, Calldata, ExitData, InstanceWrapper,
};
use crate::interfaces::{
    AddressTypeExternalFunction, BlockHashExternalFunction, CallOtherContractExternalFunction,
    ConsoleLogExternalFunction, DeployFromAddressExternalFunction, EmitExternalFunction,
    InputsExternalFunction, OutputsExternalFunction, StorageLoadExternalFunction,
    StorageStoreExternalFunction,
};
use std::sync::Arc;
use tokio::runtime::Runtime;

use super::TStore;

pub struct CustomEnv {
    pub instance: Option<InstanceWrapper>,
    pub network: BitcoinNetwork,
    pub exit_data: ExitData,
    pub storage_load_external: StorageLoadExternalFunction,
    pub storage_store_external: StorageStoreExternalFunction,
    pub call_other_contract_external: CallOtherContractExternalFunction,
    pub deploy_from_address_external: DeployFromAddressExternalFunction,
    pub console_log_external: ConsoleLogExternalFunction,
    pub emit_external: EmitExternalFunction,
    pub inputs_external: InputsExternalFunction,
    pub outputs_external: OutputsExternalFunction,
    pub address_type_external: AddressTypeExternalFunction,
    pub block_hash_external: BlockHashExternalFunction,
    pub runtime: Arc<Runtime>,
    pub store_cache: Cache,
    pub calldata: Calldata,
    pub environment_variables: Option<EnvironmentVariables>,
    pub last_call_result: CallResult,
    pub is_running_start_function: bool,
    pub transient_storage: TStore,
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
        address_type_external: AddressTypeExternalFunction,
        block_hash_external: BlockHashExternalFunction,
        runtime: Arc<Runtime>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            instance: None,
            network,
            exit_data: ExitData::default(),
            storage_load_external,
            storage_store_external,
            call_other_contract_external,
            deploy_from_address_external,
            console_log_external,
            emit_external,
            inputs_external,
            outputs_external,
            address_type_external,
            block_hash_external,
            runtime,
            store_cache: Cache::new(),
            calldata: Calldata::default(),
            environment_variables: None,
            last_call_result: CallResult::default(),
            is_running_start_function: false,
            transient_storage: TStore::new(),
        })
    }
}
