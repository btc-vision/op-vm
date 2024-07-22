use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use wasmer::RuntimeError;

use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;

pub struct ConsoleLogExternalFunction {
    tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
}

impl ConsoleLogExternalFunction {
    pub fn new(
        tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    ) -> Self {
        Self { tsfn }
    }
}

impl ConsoleLogExternalFunction {
    pub(crate) fn execute(&self, data: &[u8]) -> Result<(), RuntimeError> {
        let request = ThreadSafeJsImportResponse {
            buffer: Vec::from(data),
        };

        //let time = chrono::offset::Local::now();

        self.tsfn.call(Ok(request), ThreadsafeFunctionCallMode::NonBlocking);

        //log_time_diff(&time, "GenericExternalFunction::log");

        Ok(())
    }
}
