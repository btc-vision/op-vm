use crate::interfaces::{ExternalFunction, GenericExternalFunction, GenericFunction};
use napi::bindgen_prelude::Promise;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

#[cfg(not(feature = "use-strings-instead-of-buffers"))]
use napi::bindgen_prelude::Uint8Array;

pub struct StorageStoreExternalFunction {
    #[cfg(not(feature = "use-strings-instead-of-buffers"))]
    external_function: GenericExternalFunction<Promise<Uint8Array>>,

    #[cfg(feature = "use-strings-instead-of-buffers")]
    external_function: GenericExternalFunction<Promise<String>>,
}

impl StorageStoreExternalFunction {
    pub fn new(tsfn: GenericFunction, id: u64) -> Self {
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
