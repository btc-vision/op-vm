use crate::interfaces::napi::external_functions::GenericExternalFunction;
use crate::interfaces::napi::js_contract_manager::TsfnBuffer;
use crate::interfaces::ExternalFunction;
use std::sync::Arc;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub struct StorageLoadExternalFunction {
    external_function: GenericExternalFunction,
}

impl StorageLoadExternalFunction {
    pub fn new(tsfn: Arc<TsfnBuffer>, id: u64) -> Self {
        Self {
            external_function: GenericExternalFunction::new(tsfn, id),
        }
    }
}

impl ExternalFunction for StorageLoadExternalFunction {
    fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        self.external_function.execute(data, runtime)
    }
}
