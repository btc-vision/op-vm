use crate::interfaces::napi::external_functions::GenericExternalFunction;
// returns Buffer
use crate::interfaces::napi::js_contract_manager::TsfnBuffer;
use std::sync::Arc;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub struct InputsExternalFunction {
    external_function: GenericExternalFunction,
}

impl InputsExternalFunction {
    pub fn new(tsfn: Arc<TsfnBuffer>, id: u64) -> Self {
        Self {
            external_function: GenericExternalFunction::new(tsfn, id),
        }
    }
}

impl InputsExternalFunction {
    pub(crate) fn execute(&self, runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        // calls `execute_no_data`
        self.external_function.execute_no_data(runtime)
    }
}
