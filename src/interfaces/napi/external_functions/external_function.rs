use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub trait ExternalFunction {
    fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<Vec<u8>, RuntimeError>;
}
