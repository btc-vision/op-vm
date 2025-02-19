#[derive(Default, Debug, Clone)]
pub struct CallResult {
    pub data: Vec<u8>,
}

impl CallResult {
    pub fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }
}
