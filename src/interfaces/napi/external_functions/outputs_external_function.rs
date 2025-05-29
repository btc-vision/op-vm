use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;
use crate::interfaces::{ExternalFunctionNoData, GenericExternalFunction};
use napi::bindgen_prelude::{Buffer, Promise};
use napi::threadsafe_function::ThreadsafeFunction;
use std::sync::Arc;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub struct OutputsExternalFunction {
    external_function: GenericExternalFunction<Promise<Buffer>>,
}

impl OutputsExternalFunction {
    pub fn new(
        tsfn: Arc<
            ThreadsafeFunction<
                ThreadSafeJsImportResponse,
                Promise<Buffer>,
                ThreadSafeJsImportResponse,
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

impl OutputsExternalFunction {
    pub(crate) fn execute(&self, runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let resp = self.external_function.execute_no_data(runtime);

        resp
    }
}
