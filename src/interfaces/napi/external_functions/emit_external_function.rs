use crate::interfaces::napi::external_functions::{
    GenericExternalFunctionVoid, // for Promise<void> usage
};
use crate::interfaces::napi::js_contract_manager::TsfnVoid;
use crate::interfaces::ExternalFunctionNoResponse;
use std::sync::Arc;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub struct EmitExternalFunction {
    // Note that we now use the "Void" variant for Promise<void>.
    external_function: GenericExternalFunctionVoid,
}

impl EmitExternalFunction {
    pub fn new(tsfn: Arc<TsfnVoid>, id: u64) -> Self {
        Self {
            external_function: GenericExternalFunctionVoid::new(tsfn, id),
        }
    }
}

impl EmitExternalFunction {
    pub(crate) fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<(), RuntimeError> {
        // Now calls `execute_no_response` on the `GenericExternalFunctionVoid`.
        self.external_function.execute_no_response(data, runtime)
    }
}
