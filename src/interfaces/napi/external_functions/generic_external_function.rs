use napi::bindgen_prelude::{BigInt, Buffer};
use std::sync::Arc;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use crate::interfaces::napi::js_contract_manager::{TsfnBuffer, TsfnVoid};
use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;
use crate::interfaces::{ExternalFunction, ExternalFunctionNoData, ExternalFunctionNoResponse};

pub struct GenericExternalFunction {
    tsfn: Arc<TsfnBuffer>,
    contract_id: u64,
}

impl GenericExternalFunction {
    pub fn new(tsfn: Arc<TsfnBuffer>, contract_id: u64) -> Self {
        Self { tsfn, contract_id }
    }

    /// For code that needs to pass [u8] and get back Vec<u8> from a `Promise<Buffer>`
    pub fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let request = ThreadSafeJsImportResponse {
            buffer: data.to_vec(),
            contract_id: BigInt::from(self.contract_id),
        };

        let fut = async move {
            // call_async => yields a Promise<Buffer>
            let buf: Buffer = self
                .tsfn
                .call_async(Ok(request))
                .await
                .map_err(|err| RuntimeError::new(err.reason))?;

            Ok(buf.to_vec())
        };
        runtime.block_on(fut)
    }

    /// For code that needs [no input data] but returns a Vec<u8> from JS
    pub fn execute_no_data(&self, runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let request = ThreadSafeJsImportResponse {
            buffer: Vec::new(),
            contract_id: BigInt::from(self.contract_id),
        };

        let fut = async move {
            let buf: Buffer = self
                .tsfn
                .call_async(Ok(request))
                .await
                .map_err(|err| RuntimeError::new(err.reason))?;

            Ok(buf.to_vec())
        };
        runtime.block_on(fut)
    }
}

impl ExternalFunction for GenericExternalFunction {
    fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        self.execute(data, runtime)
    }
}

impl ExternalFunctionNoData for GenericExternalFunction {
    fn execute_no_data(&self, runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        self.execute_no_data(runtime)
    }
}

pub struct GenericExternalFunctionVoid {
    tsfn: Arc<TsfnVoid>,
    contract_id: u64,
}

impl GenericExternalFunctionVoid {
    pub fn new(tsfn: Arc<TsfnVoid>, contract_id: u64) -> Self {
        Self { tsfn, contract_id }
    }

    /// For code that passes data but expects no data back from JS.
    pub fn execute_no_response(&self, data: &[u8], runtime: &Runtime) -> Result<(), RuntimeError> {
        let request = ThreadSafeJsImportResponse {
            buffer: data.to_vec(),
            contract_id: BigInt::from(self.contract_id),
        };

        let fut = async move {
            self.tsfn
                .call_async(Ok(request))
                .await
                .map_err(|err| RuntimeError::new(err.reason))?;

            Ok(())
        };

        runtime.block_on(fut)
    }
}

impl ExternalFunctionNoResponse for GenericExternalFunctionVoid {
    fn execute_no_response(&self, data: &[u8], runtime: &Runtime) -> Result<(), RuntimeError> {
        self.execute_no_response(data, runtime)
    }
}
