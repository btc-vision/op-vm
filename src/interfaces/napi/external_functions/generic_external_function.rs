use napi::bindgen_prelude::{BigInt, Buffer, FromNapiValue, Promise, Unknown};
use napi::threadsafe_function::ThreadsafeFunction;
use napi::Status;
use std::sync::Arc;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;
use crate::interfaces::{ExternalFunction, ExternalFunctionNoData, ExternalFunctionNoResponse};

/// Generic wrapper around a `ThreadsafeFunction` whose JavaScript promise
/// resolves to any N-API value `R` (`Buffer`, `()`, …).
pub struct GenericExternalFunction<Return: 'static + FromNapiValue = Unknown<'static>> {
    tsfn: Arc<
        ThreadsafeFunction<
            ThreadSafeJsImportResponse,
            Return,
            ThreadSafeJsImportResponse,
            Status,
            true,
            false,
            128,
        >,
    >,
    contract_id: u64,
}

impl<R> GenericExternalFunction<R>
where
    R: FromNapiValue,
{
    pub fn new(
        tsfn: Arc<
            ThreadsafeFunction<
                ThreadSafeJsImportResponse,
                R,
                ThreadSafeJsImportResponse,
                Status,
                true,
                false,
                128,
            >,
        >,
        contract_id: u64,
    ) -> Self {
        Self { tsfn, contract_id }
    }

    fn make_request(&self, buffer: Vec<u8>) -> ThreadSafeJsImportResponse {
        ThreadSafeJsImportResponse {
            buffer,
            contract_id: BigInt::from(self.contract_id),
        }
    }
}

impl ExternalFunction for GenericExternalFunction<Promise<Buffer>> {
    fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let tsfn = self.tsfn.clone();
        let request = self.make_request(data.to_vec());

        let fut = async move {
            let promise = tsfn.call_async(Ok(request)).await;

            let promise = match promise {
                Ok(promise) => promise,
                Err(e) => {
                    return Err(RuntimeError::new(e.reason.clone()));
                }
            };

            let buffer = promise
                .await
                .map_err(|e| RuntimeError::new(e.reason.clone()))?;
            Ok(buffer.to_vec())
        };

        runtime.block_on(fut)
    }
}

impl ExternalFunctionNoData for GenericExternalFunction<Promise<Buffer>> {
    fn execute_no_data(&self, runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let tsfn = self.tsfn.clone();
        let request = self.make_request(Vec::new());

        let fut = async move {
            let promise = tsfn
                .call_async(Ok(request))
                .await
                .map_err(|e| RuntimeError::new(e.reason.clone()))?;

            let buffer = promise
                .await
                .map_err(|e| RuntimeError::new(e.reason.clone()))?;
            Ok(buffer.to_vec())
        };

        runtime.block_on(fut)
    }
}

impl ExternalFunctionNoResponse for GenericExternalFunction<Promise<()>> {
    fn execute_no_response(&self, data: &[u8], runtime: &Runtime) -> Result<(), RuntimeError> {
        let tsfn = self.tsfn.clone();
        let request = self.make_request(data.to_vec());

        let fut = async move {
            let promise = tsfn
                .call_async(Ok(request))
                .await
                .map_err(|e| RuntimeError::new(e.reason.clone()))?;

            promise
                .await
                .map_err(|e| RuntimeError::new(e.reason.clone()))
        };

        runtime.block_on(fut)
    }
}
