use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;
use crate::interfaces::{ExternalFunctionNoData, GenericExternalFunction};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub struct OutputsExternalFunction {
    external_function: GenericExternalFunction,
}

impl OutputsExternalFunction {
    pub fn new(
        tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
        id: u64,
    ) -> Self {
        Self {
            external_function: GenericExternalFunction::new(tsfn, id),
        }
    }
}

impl OutputsExternalFunction {
    pub(crate) fn execute(&self, runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let resp = self.external_function.execute_no_data(runtime);

        resp
    }
}
