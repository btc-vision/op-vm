use crate::domain::common::Address;

#[derive(Default, Debug, Clone)]
pub struct EnvironmentVariables {
    block_hash: Vec<u8>,
    block_number: u64,
    block_median_time: u64,
    tx_id: Vec<u8>,
    tx_hash: Vec<u8>,
    contract_address: Address,
    contract_deployer: Address,
    caller: Address,
    origin: Address,
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
        result
    }
}
