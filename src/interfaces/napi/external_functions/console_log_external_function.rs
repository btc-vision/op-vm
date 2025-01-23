use crate::interfaces::napi::external_functions::GenericExternalFunctionVoid;
use crate::interfaces::napi::js_contract_manager::TsfnVoid;
use std::sync::Arc;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub struct ConsoleLogExternalFunction {
    // This uses the "void" variant => Promise<void>
    external_function: GenericExternalFunctionVoid,
}

impl ConsoleLogExternalFunction {
    pub fn new(tsfn: Arc<TsfnVoid>, id: u64) -> Self {
        Self {
            external_function: GenericExternalFunctionVoid::new(tsfn, id),
        }
    }
}

impl ConsoleLogExternalFunction {
    pub(crate) fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<(), RuntimeError> {
        self.external_function.execute_no_response(data, runtime)
    }
}
