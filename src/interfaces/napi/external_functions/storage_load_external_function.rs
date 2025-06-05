use napi::bindgen_prelude::{Buffer, Promise};
use napi::threadsafe_function::ThreadsafeFunction;
use std::sync::Arc;
use napi::Status;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use crate::interfaces::napi::external_functions::GenericExternalFunction;
use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;
use crate::interfaces::ExternalFunction;

pub struct StorageLoadExternalFunction {
    external_function: GenericExternalFunction<Promise<Buffer>>,
}

impl StorageLoadExternalFunction {
    pub fn new(
        tsfn: Arc<
            ThreadsafeFunction<
                ThreadSafeJsImportResponse,
                Promise<Buffer>,
                ThreadSafeJsImportResponse,
                Status,
                true,
                false,
                128,
            >,
        >,
        id: u64,
    ) -> Self {
        Self {
            external_function: GenericExternalFunction::new(tsfn, id),
        }
    }
}

impl ExternalFunction for StorageLoadExternalFunction {
    fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        //let time = chrono::offset::Local::now();
        /*if self.is_destroyed.load(Ordering::SeqCst) {
            return Err(RuntimeError::new("Contract is being destroyed"));
        }*/

        //self.pending_calls.fetch_add(1, Ordering::SeqCst);
        let result = self.external_function.execute(data, runtime);
        //self.pending_calls.fetch_sub(1, Ordering::SeqCst);
        //log_time_diff(&time, "GenericExternalFunction::load");

        result
    }
}
