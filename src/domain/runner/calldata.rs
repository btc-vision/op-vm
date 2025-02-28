#[derive(Default, Debug, Clone)]
pub struct Calldata {
    data: Vec<u8>,
}

impl Calldata {
    pub fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}
