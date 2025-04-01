use napi::bindgen_prelude::{BigInt, Promise};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;

#[napi(object)]
pub struct AccountTypeResponse {
    pub account_type: u32,
    pub is_address_warm: bool,
}

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
    pub fn execute(
        &self,
        address_hash: &[u8],
        runtime: &Runtime,
    ) -> Result<AccountTypeResponse, RuntimeError> {
        let request = ThreadSafeJsImportResponse {
            buffer: address_hash.to_vec(),
            contract_id: BigInt::from(self.contract_id),
        };

        let result = async move {
            let response: Result<Promise<AccountTypeResponse>, RuntimeError> = self
                .tsfn
                .call_async(Ok(request))
                .await
                .map_err(|e| RuntimeError::new(e.reason));

            let promise = response?;

            let data = promise.await.map_err(|e| RuntimeError::new(e.reason))?;

            Ok(data)
        };

        let response = runtime.block_on(result);

        response
    }
}
