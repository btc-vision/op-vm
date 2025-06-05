use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;
use crate::interfaces::{ExternalFunction, GenericExternalFunction};
use napi::bindgen_prelude::{Buffer, Promise};
use napi::threadsafe_function::ThreadsafeFunction;
use napi::Status;
use std::sync::Arc;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub struct StorageStoreExternalFunction {
    external_function: GenericExternalFunction<Promise<Buffer>>,
}

impl StorageStoreExternalFunction {
    pub fn new(
        tsfn: Arc<
            ThreadsafeFunction<
                ThreadSafeJsImportResponse,
                Promise<Buffer>,
                ThreadSafeJsImportResponse,
                Status,
                true,
                true,
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

impl ExternalFunction for StorageStoreExternalFunction {
    fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        //let time = chrono::offset::Local::now();
        let resp = self.external_function.execute(data, runtime);

        //log_time_diff(&time, "GenericExternalFunction::store");

        resp
    }
}
