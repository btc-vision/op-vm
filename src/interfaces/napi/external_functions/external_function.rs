use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub trait ExternalFunction {
    fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<Vec<u8>, RuntimeError>;
}

pub trait ExternalFunctionNoData {
    fn execute_no_data(&self, runtime: &Runtime) -> Result<Vec<u8>, RuntimeError>;
}
