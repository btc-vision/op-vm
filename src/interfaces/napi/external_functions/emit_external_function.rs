use napi::bindgen_prelude::BigInt;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use wasmer::RuntimeError;

use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;

pub struct EmitExternalFunction {
    tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    id: u64,
}

impl EmitExternalFunction {
    pub fn new(
        tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
        id: u64,
    ) -> Self {
        Self { tsfn, id }
    }
}

impl EmitExternalFunction {
    pub(crate) fn execute(&self, data: &[u8]) -> Result<(), RuntimeError> {
        let request = ThreadSafeJsImportResponse {
            buffer: Vec::from(data),
            contract_id: BigInt::from(self.id),
        };

        self.tsfn.call(Ok(request), ThreadsafeFunctionCallMode::NonBlocking);

        Ok(())
    }
}
