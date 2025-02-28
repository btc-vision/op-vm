use crate::domain::runner::ExitData;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use wasmer::RuntimeError;

#[derive(Clone, Debug)]
pub enum ExitResult {
    Ok(ExitData),
    Err(RuntimeError)
}

impl Error for ExitResult {}

impl Display for ExitResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ExitResult::Ok(_) => write!(f, "Exit data: {}", self),
            ExitResult::Err(_) => write!(f, "Error: {}", self),
        }
    }
}
