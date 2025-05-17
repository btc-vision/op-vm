use crate::domain::runner::ProvenState;
use bitcoin::hex::DisplayHex;
use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct ExitData {
    pub status: u32,
    pub data: Vec<u8>,
    pub gas_used: u64,

    pub proofs: Vec<ProvenState>,
}

impl ExitData {
    pub fn new(status: u32, gas_used: u64, data: &[u8], proofs: Vec<ProvenState>) -> Self {
        Self {
            status,
            data: data.to_vec(),
            gas_used,
            proofs,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.status == 0
    }
}

impl Display for ExitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "status: {}, gas_used: {}, data: {}, proofs: {}",
            self.status,
            self.gas_used,
            self.data.to_lower_hex_string(),
            self.proofs
                .iter()
                .map(|p| format!(
                    "{{proof: {}, vk: {}}}",
                    p.proof.to_lower_hex_string(),
                    p.vk.to_lower_hex_string()
                ))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
