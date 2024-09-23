use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;
use crate::interfaces::{ExternalFunction, GenericExternalFunction};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub struct StorageStoreExternalFunction {
    external_function: GenericExternalFunction,
}

impl StorageStoreExternalFunction {
    pub fn new(
        tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    ) -> Self {
        Self {
            external_function: GenericExternalFunction::new(tsfn),
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


/*pub struct StorageStoreExternalFunction {
    tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
}

impl StorageStoreExternalFunction {
    pub fn new(
        tsfn: ThreadsafeFunction<ThreadSafeJsImportResponse, ErrorStrategy::CalleeHandled>,
    ) -> Self {
        Self { tsfn }
    }
}

impl StorageStoreExternalFunction {
    pub(crate) fn execute(&self, data: &[u8]) -> Result<Vec<u8>, RuntimeError> {
        let request = ThreadSafeJsImportResponse {
            buffer: Vec::from(data),
        };

        //let time = chrono::offset::Local::now();

        let resp = self.tsfn.call_with_return_value(Ok(request), ThreadsafeFunctionCallMode::NonBlocking, |env, data| {
            let data = data.buffer;
            Ok(env.create_buffer_with_data(data)?.into_raw())
        });

        //log_time_diff(&time, "GenericExternalFunction::log");

        resp
    }
}*/
