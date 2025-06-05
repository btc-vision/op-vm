use napi::bindgen_prelude::{Buffer, Promise};
use napi::threadsafe_function::ThreadsafeFunction;
use std::sync::Arc;
use napi::Status;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use crate::interfaces::napi::external_functions::GenericExternalFunction;
use crate::interfaces::napi::thread_safe_js_import_response::ThreadSafeJsImportResponse;
use crate::interfaces::ExternalFunction;

pub struct DeployFromAddressExternalFunction {
    external_function: GenericExternalFunction<Promise<Buffer>>,
}

impl DeployFromAddressExternalFunction {
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

impl ExternalFunction for DeployFromAddressExternalFunction {
    fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        //let time = chrono::offset::Local::now();
        let resp = self.external_function.execute(data, runtime);

        //log_time_diff(&time, "GenericExternalFunction::deploy");

        resp
    }
}
