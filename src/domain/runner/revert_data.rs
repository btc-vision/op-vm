use bitcoin::hex::DisplayHex;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct RevertData {
    pub data: Vec<u8>,
}

impl RevertData {
    pub fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }
}

impl Display for RevertData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data.to_lower_hex_string())
    }
}
