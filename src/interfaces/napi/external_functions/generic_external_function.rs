use napi::bindgen_prelude::{BigInt, Buffer, Promise};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;
use crate::interfaces::ExternalFunction;

pub struct GenericExternalFunction {
    tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    contract_id: u64,
}

impl GenericExternalFunction {
    pub fn new(
        tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
        contract_id: u64,
    ) -> Self {
        Self { tsfn, contract_id }
    }
}

impl ExternalFunction for GenericExternalFunction {
    fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let request = ThreadSafeJsImportResponse {
            buffer: Vec::from(data),
            contract_id: BigInt::from(self.contract_id),
        };

        let deploy = async move {
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

            Ok(data.into())
        };

        let response = runtime.block_on(deploy);

        response
    }
}
