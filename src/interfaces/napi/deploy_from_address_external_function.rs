use napi::bindgen_prelude::{Buffer, Promise};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;

pub struct DeployFromAddressExternalFunction {
    tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
}

impl DeployFromAddressExternalFunction {
    pub fn new(
        tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    ) -> Self {
        Self { tsfn }
    }

    pub fn execute(&self, data: &[u8]) -> Result<Vec<u8>, RuntimeError> {
        let request: ThreadSafeJsImportResponse = ThreadSafeJsImportResponse {
            buffer: Vec::from(data),
        };

        let deploy = async move {
            let response: Result<Promise<Buffer>, RuntimeError> = self.tsfn
                .call_async(Ok(request))
                .await
                .map_err(|_e| RuntimeError::new("Error calling load function"));

            let promise: Promise<Buffer> = response?;

            let data: Buffer = promise
                .await
                .map_err(|_e| RuntimeError::new("Error awaiting promise"))?;

            let data: Vec<u8> = data.to_vec();
            Ok(data.into())
        };

        let rt: Runtime = Runtime::new().unwrap();
        let response: Result<Vec<u8>, RuntimeError> = rt.block_on(deploy);
        response
    }
}
