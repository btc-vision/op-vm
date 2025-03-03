use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ExitSignal {
    status: u32,
}

impl ExitSignal {
    pub fn new(status: u32) -> Self {
        Self {
            status,
        }
    }
}

impl Error for ExitSignal {}

impl Display for ExitSignal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Contract exited with status {}", self.status)
    }
}
