use napi::bindgen_prelude::{BigInt, Promise};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;

pub struct AccountTypeExternalFunction {
    tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    contract_id: u64,
}

impl AccountTypeExternalFunction {
    pub fn new(
        tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
        id: u64,
    ) -> Self {
        Self {
            tsfn,
            contract_id: id,
        }
    }
}

impl AccountTypeExternalFunction {
    pub fn execute(&self, address_hash: &[u8], runtime: &Runtime) -> Result<u32, RuntimeError> {
        let request = ThreadSafeJsImportResponse {
            buffer: address_hash.to_vec(),
            contract_id: BigInt::from(self.contract_id),
        };

        let deploy = async move {
            let response: Result<Promise<napi::JsNumber>, RuntimeError> = self
                .tsfn
                .call_async(Ok(request))
                .await
                .map_err(|e| RuntimeError::new(e.reason));

            let promise = response?;

            let data = promise.await.map_err(|e| RuntimeError::new(e.reason))?;

            Ok(data
                .get_uint32()
                .map_err(|_err| RuntimeError::new("Cannot convert result to u32"))?)
        };

        let response = runtime.block_on(deploy);

        response
    }
}
