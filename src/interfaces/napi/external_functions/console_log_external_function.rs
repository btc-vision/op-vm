use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;
use crate::interfaces::{ExternalFunctionNoResponse, GenericExternalFunction};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub struct ConsoleLogExternalFunction {
    external_function: GenericExternalFunction,
}

impl ConsoleLogExternalFunction {
    pub fn new(
        tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
        id: u64,
    ) -> Self {
        Self { external_function: GenericExternalFunction::new(tsfn, id) }
    }
}

impl ConsoleLogExternalFunction {
    pub(crate) fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<(), RuntimeError> {
        let resp = self.external_function.execute_no_response(data, runtime);

        resp
    }
}
