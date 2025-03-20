use napi::{
    bindgen_prelude::{BigInt, Buffer, Promise},
    threadsafe_function::{ErrorStrategy, ThreadsafeFunction},
};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

#[napi(object)]
pub struct BlockHashRequest {
    pub block_id: BigInt,
    pub contract_id: BigInt,
}

pub struct BlockHashExternalFunction {
    tsfn: ThreadsafeFunction<BlockHashRequest, ErrorStrategy::CalleeHandled>,
    contract_id: u64,
}

impl BlockHashExternalFunction {
    pub fn new(
        tsfn: ThreadsafeFunction<BlockHashRequest, ErrorStrategy::CalleeHandled>,
        contract_id: u64,
    ) -> Self {
        Self { tsfn, contract_id }
    }

    pub fn execute(&self, block_id: u64, runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let request = BlockHashRequest {
            block_id: block_id.into(),
            contract_id: BigInt::from(self.contract_id),
        };

        let deploy = async move {
            let response: Result<Promise<Buffer>, RuntimeError> = self
                .tsfn
                .call_async(Ok(request))
                .await
                .map_err(|e| RuntimeError::new(e.reason));

            let promise = response?;

            let data = promise.await.map_err(|e| RuntimeError::new(e.reason))?;
            Ok(data.to_vec().into())
        };

        let response = runtime.block_on(deploy);

        response
    }
}
