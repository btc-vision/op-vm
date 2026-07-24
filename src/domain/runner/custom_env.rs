use crate::domain::runner::environment_variables::EnvironmentVariables;
use crate::domain::runner::{
    BitcoinNetwork, CallResult, Calldata, ConsensusFlags, ExitData, HardFork, InstanceWrapper,
    MAX_MEMORY_COPY_SIZE,
};
use crate::interfaces::{
    AccountTypeExternalFunction, BlockHashExternalFunction, CallOtherContractExternalFunction,
    ConsoleLogExternalFunction, DeployFromAddressExternalFunction, EmitExternalFunction,
    InputsExternalFunction, MLDSALoadExternalFunction, OutputsExternalFunction,
    StorageLoadExternalFunction, StorageStoreExternalFunction, UpdateFromAddressExternalFunction,
};
use std::sync::Arc;
use tokio::runtime::Runtime;
use wasmer::{AsStoreMut, RuntimeError};

use super::TransientStorage;

#[derive(Clone, Debug, Default)]
pub struct ProvenState {
    pub proof: Vec<u8>,
    pub vk: Vec<u8>,
}

pub struct CustomEnv {
    pub instance: Option<InstanceWrapper>,
    pub network: BitcoinNetwork,
    pub consensus_flags: ConsensusFlags,
    pub exit_data: ExitData,
    pub storage_load_external: StorageLoadExternalFunction,
    pub storage_store_external: StorageStoreExternalFunction,
    pub call_other_contract_external: CallOtherContractExternalFunction,
    pub deploy_from_address_external: DeployFromAddressExternalFunction,
    pub update_from_address_external: UpdateFromAddressExternalFunction,
    pub console_log_external: ConsoleLogExternalFunction,
    pub emit_external: EmitExternalFunction,
    pub inputs_external: InputsExternalFunction,
    pub outputs_external: OutputsExternalFunction,
    pub account_type_external: AccountTypeExternalFunction,
    pub block_hash_external: BlockHashExternalFunction,
    pub mldsa_load_external: MLDSALoadExternalFunction,
    pub runtime: Arc<Runtime>,
    pub calldata: Calldata,
    pub environment_variables: Option<EnvironmentVariables>,
    pub last_call_result: CallResult,
    pub is_running_start_function: bool,
    pub transient_storage: TransientStorage,
    pub max_pages: u32,
    pub hard_fork: HardFork,

    #[allow(dead_code)]
    pub return_proofs: bool,
    pub proofs: Vec<ProvenState>,
}

impl CustomEnv {
    pub fn new(
        network: BitcoinNetwork,
        consensus_flags: ConsensusFlags,
        storage_load_external: StorageLoadExternalFunction,
        storage_store_external: StorageStoreExternalFunction,
        call_other_contract_external: CallOtherContractExternalFunction,
        deploy_from_address_external: DeployFromAddressExternalFunction,
        update_from_address_external: UpdateFromAddressExternalFunction,
        console_log_external: ConsoleLogExternalFunction,
        emit_external: EmitExternalFunction,
        inputs_external: InputsExternalFunction,
        outputs_external: OutputsExternalFunction,
        account_type_external: AccountTypeExternalFunction,
        block_hash_external: BlockHashExternalFunction,
        mldsa_load_external: MLDSALoadExternalFunction,
        runtime: Arc<Runtime>,
        max_pages: u32,
        hard_fork: HardFork,
        return_proofs: bool,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            instance: None,
            network,
            consensus_flags,
            exit_data: ExitData::default(),
            storage_load_external,
            storage_store_external,
            call_other_contract_external,
            deploy_from_address_external,
            update_from_address_external,
            console_log_external,
            emit_external,
            inputs_external,
            outputs_external,
            account_type_external,
            block_hash_external,
            mldsa_load_external,
            runtime,
            calldata: Calldata::default(),
            environment_variables: None,
            last_call_result: CallResult::default(),
            is_running_start_function: false,
            transient_storage: TransientStorage::new(),
            max_pages,
            hard_fork,

            return_proofs,
            proofs: Vec::new(),
        })
    }

    pub fn set_consensus_flags(&mut self, consensus_flags: ConsensusFlags) {
        self.consensus_flags = consensus_flags;
    }

    pub fn is_strict_memory_metering_enabled(&self) -> bool {
        self.consensus_flags
            .contains(ConsensusFlags::STRICT_MEMORY_METERING)
    }

    pub fn charge_gas(
        &self,
        instance: &InstanceWrapper,
        store: &mut impl AsStoreMut,
        gas_cost: u64,
    ) -> Result<(), RuntimeError> {
        if self.is_strict_memory_metering_enabled() {
            instance.try_use_gas(store, gas_cost)
        } else {
            instance.use_gas(store, gas_cost);
            Ok(())
        }
    }

    pub fn ensure_host_copy_length(
        &self,
        length: u32,
        operation: &str,
    ) -> Result<(), RuntimeError> {
        self.ensure_host_copy_size(length as usize, operation)
    }

    pub fn ensure_host_copy_size(
        &self,
        length: usize,
        operation: &str,
    ) -> Result<(), RuntimeError> {
        if self.is_strict_memory_metering_enabled() && length > MAX_MEMORY_COPY_SIZE as usize {
            return Err(RuntimeError::new(format!(
                "{} length exceeds maximum allowed size",
                operation
            )));
        }

        Ok(())
    }
}
