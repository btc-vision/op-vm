use napi::bindgen_prelude::BigInt;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use wasmer::RuntimeError;

use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;

pub struct FlushEventsExternalFunction {
    tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    id: u64,
}

impl FlushEventsExternalFunction {
    pub fn new(
        tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
        id: u64,
    ) -> Self {
        Self { tsfn, id }
    }
}

impl FlushEventsExternalFunction {
    pub(crate) fn execute(&self, data: &[u8]) -> Result<(), RuntimeError> {
        let request = ThreadSafeJsImportResponse {
            buffer: Vec::from(data),
            contract_id: BigInt::from(self.id),
        };

        //let time = chrono::offset::Local::now();

        self.tsfn
            .call(Ok(request), ThreadsafeFunctionCallMode::NonBlocking);

        //log_time_diff(&time, "GenericExternalFunction::log");

        Ok(())
    }
}
