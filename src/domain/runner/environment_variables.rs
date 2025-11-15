use crate::domain::common::Address;
use crate::domain::runner::ConsensusFlags;

#[derive(Default, Debug, Clone)]
pub struct EnvironmentVariables {
    consensus_flags: ConsensusFlags,
    block_hash: Vec<u8>,
    block_number: u64,
    block_median_time: u64,
    tx_id: Vec<u8>,
    tx_hash: Vec<u8>,
    contract_address: Address,
    contract_deployer: Address,
    caller: Address,
    origin: Address,
    origin_tweaked_public_key: Vec<u8>,
    chain_id: Vec<u8>,
    protocol_id: Vec<u8>,
}

impl EnvironmentVariables {
    pub fn new(
        block_hash: &[u8],
        block_number: u64,
        block_median_time: u64,
        tx_id: &[u8],
        tx_hash: &[u8],
        contract_address: Address,
        contract_deployer: Address,
        caller: Address,
        origin: Address,
        chain_id: &[u8],
        protocol_id: &[u8],
        origin_tweaked_public_key: &[u8],
        consensus_flags: ConsensusFlags,
    ) -> Self {
        Self {
            block_hash: block_hash.to_vec(),
            block_number,
            block_median_time,
            tx_id: tx_id.to_vec(),
            tx_hash: tx_hash.to_vec(),
            contract_address,
            contract_deployer,
            caller,
            origin,
            chain_id: chain_id.to_vec(),
            protocol_id: protocol_id.to_vec(),
            origin_tweaked_public_key: origin_tweaked_public_key.to_vec(),
            consensus_flags,
        }
    }

    pub fn serialize_for_contract(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.block_hash);
        result.extend_from_slice(&self.block_number.to_be_bytes());
        result.extend_from_slice(&self.block_median_time.to_be_bytes());
        result.extend_from_slice(&self.tx_id);
        result.extend_from_slice(&self.tx_hash);
        result.extend_from_slice(&self.contract_address.to_bytes());
        result.extend_from_slice(&self.contract_deployer.to_bytes());
        result.extend_from_slice(&self.caller.to_bytes());
        result.extend_from_slice(&self.origin.to_bytes());
        result.extend_from_slice(&self.chain_id);
        result.extend_from_slice(&self.protocol_id);
        result.extend_from_slice(&self.origin_tweaked_public_key);
        result.extend_from_slice(&self.consensus_flags.to_be_bytes());
        result
    }

    pub fn is_consensus_flag_set(&self, flag: ConsensusFlags) -> bool {
        self.consensus_flags.contains(flag)
    }
}
