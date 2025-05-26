use napi::bindgen_prelude::Promise;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use crate::interfaces::napi::external_functions::GenericExternalFunction;
use crate::interfaces::{ExternalFunction, GenericFunction};

#[cfg(not(feature = "use-strings-instead-of-buffers"))]
use napi::bindgen_prelude::Buffer;

pub struct StorageLoadExternalFunction {
    #[cfg(not(feature = "use-strings-instead-of-buffers"))]
    external_function: GenericExternalFunction<Promise<Buffer>>,

    #[cfg(feature = "use-strings-instead-of-buffers")]
    external_function: GenericExternalFunction<Promise<String>>,
}

impl StorageLoadExternalFunction {
    pub fn new(tsfn: GenericFunction, id: u64) -> Self {
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
