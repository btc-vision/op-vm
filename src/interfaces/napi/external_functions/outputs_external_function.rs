use crate::interfaces::napi::external_functions::GenericExternalFunction;
use crate::interfaces::napi::js_contract_manager::TsfnBuffer;
use std::sync::Arc;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub struct OutputsExternalFunction {
    external_function: GenericExternalFunction,
}

impl OutputsExternalFunction {
    pub fn new(tsfn: Arc<TsfnBuffer>, id: u64) -> Self {
        Self {
            external_function: GenericExternalFunction::new(tsfn, id),
        }
    }
}

impl OutputsExternalFunction {
    pub(crate) fn execute(&self, runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        self.external_function.execute_no_data(runtime)
    }
}
