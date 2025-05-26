use crate::interfaces::{ExternalFunctionNoData, GenericExternalFunction, GenericFunction};
use napi::bindgen_prelude::Promise;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

#[cfg(not(feature = "use-strings-instead-of-buffers"))]
use napi::bindgen_prelude::Uint8Array;

pub struct InputsExternalFunction {
    #[cfg(not(feature = "use-strings-instead-of-buffers"))]
    external_function: GenericExternalFunction<Promise<Uint8Array>>,

    #[cfg(feature = "use-strings-instead-of-buffers")]
    external_function: GenericExternalFunction<Promise<String>>,
}

impl InputsExternalFunction {
    pub fn new(tsfn: GenericFunction, id: u64) -> Self {
        Self {
            external_function: GenericExternalFunction::new(tsfn, id),
        }
    }
}

impl InputsExternalFunction {
    pub(crate) fn execute(&self, runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let resp = self.external_function.execute_no_data(runtime);

        resp
    }
}
