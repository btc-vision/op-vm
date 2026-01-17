use std::sync::Arc;

use crate::interfaces::ExternalFunction;
use neon::prelude::{Channel, JsFunction, Root};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub struct GenericExternalFunctionDummy<R> {
    name: String,
    #[allow(dead_code)]
    contract_id: u64,
    responder: Arc<dyn Fn(&[u8]) -> Result<R, RuntimeError> + Send + Sync>,
}

impl<R> GenericExternalFunctionDummy<R>
where
    R: Send + 'static,
{
    /// Build a dummy with an arbitrary responder.
    ///
    /// ```rust
    /// /*let echo = GenericExternalFunctionDummy::new(
    ///     "echo",
    ///     42,
    ///     |data| Ok(data.to_vec())
    /// );*/
    /// ```
    pub fn new<F, N>(name: N, contract_id: u64, responder: F) -> Self
    where
        N: Into<String>,
        F: Fn(&[u8]) -> Result<R, RuntimeError> + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            contract_id,
            responder: Arc::new(responder),
        }
    }

    /// Mirrors `GenericExternalFunction::execute`.
    pub fn execute(&self, call_data: &[u8], _rt: &Runtime) -> Result<R, RuntimeError> {
        (self.responder)(call_data)
    }
}

impl<R> ExternalFunction<R> for GenericExternalFunctionDummy<R>
where
    R: Send + 'static + std::marker::Sync,
{
    fn name(&self) -> String {
        self.name.clone()
    }

    // These two are **never used** inside unit-tests, so we just make them
    // unreachable. `!` (the *never* type) coerces to any return type.
    fn handle(&self) -> Arc<Root<JsFunction>> {
        unreachable!("`handle()` should not be called in pure Rust unit-tests")
    }

    fn channel(&self) -> Channel {
        unreachable!("`channel()` should not be called in pure Rust unit-tests")
    }
}
