use napi::bindgen_prelude::Promise;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use crate::interfaces::napi::external_functions::GenericExternalFunction;
use crate::interfaces::{ExternalFunction, GenericFunction};

pub struct CallOtherContractExternalFunction {
    #[cfg(not(feature = "use-strings-instead-of-buffers"))]
    external_function: GenericExternalFunction<Promise<Buffer>>,
    #[cfg(feature = "use-strings-instead-of-buffers")]
    external_function: GenericExternalFunction<Promise<String>>,
}

impl CallOtherContractExternalFunction {
    pub fn new(tsfn: GenericFunction, id: u64) -> Self {
        Self {
            external_function: GenericExternalFunction::new(tsfn, id),
        }
    }
}

impl ExternalFunction for CallOtherContractExternalFunction {
    fn execute(&self, data: &[u8], runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        //let time = chrono::offset::Local::now();
        let resp = self.external_function.execute(data, runtime);

        //log_time_diff(&time, "GenericExternalFunction::call");

        resp
    }
}
