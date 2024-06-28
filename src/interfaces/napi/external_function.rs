use wasmer::RuntimeError;

pub trait ExternalFunction {
    fn execute(&self, data: &[u8]) -> Result<Vec<u8>, RuntimeError>;
}
