use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use wasmer::RuntimeError;

use crate::interfaces::ExternalFunction;
use crate::interfaces::napi::generic_external_function::GenericExternalFunction;
use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;

pub struct ConsoleLogExternalFunction {
    external_function: GenericExternalFunction,
}

impl ConsoleLogExternalFunction {
    pub fn new(
        tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    ) -> Self {
        Self {
            external_function: GenericExternalFunction::new(tsfn),
        }
    }
}

impl ExternalFunction for ConsoleLogExternalFunction {
    fn execute(&self, data: &[u8]) -> Result<Vec<u8>, RuntimeError> {
        self.external_function.execute(data)
    }
}
