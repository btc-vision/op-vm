use napi::bindgen_prelude::{Buffer, Promise};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use crate::domain::vm::log_time_diff;
use crate::interfaces::ExternalFunction;
use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;

pub struct GenericExternalFunction {
    tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
}

impl GenericExternalFunction {
    pub fn new(
        tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    ) -> Self {
        Self { tsfn }
    }
}

impl ExternalFunction for GenericExternalFunction {
    fn execute(&self, data: &[u8]) -> Result<Vec<u8>, RuntimeError> {
        let request = ThreadSafeJsImportResponse {
            buffer: Vec::from(data),
        };

        let deploy = async move {
            let time = chrono::offset::Local::now();
            let response: Result<Promise<Buffer>, RuntimeError> = self
                .tsfn
                .call_async(Ok(request))
                .await
                .map_err(|e| RuntimeError::new(e.reason));

            let promise = response?;

            let data = promise
                .await
                .map_err(|e| RuntimeError::new(e.reason))?;

            let data = data.to_vec();

            log_time_diff(&time, "GenericExternalFunction::execute");

            Ok(data.into())
        };

        let rt = Runtime::new().unwrap();
        let response = rt.block_on(deploy);
        response
    }
}
