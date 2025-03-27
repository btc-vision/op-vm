use bitcoin::hex::DisplayHex;
use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct ExitData {
    pub status: u32,
    pub data: Vec<u8>,
    pub gas_used: u64,
}

impl ExitData {
    pub fn new(status: u32, gas_used: u64, data: &[u8]) -> Self {
        Self {
            status,
            data: data.to_vec(),
            gas_used,
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
            "status: {}, gas_used: {}, data: {}",
            self.status,
            self.gas_used,
            self.data.to_lower_hex_string(),
        )
    }
}
