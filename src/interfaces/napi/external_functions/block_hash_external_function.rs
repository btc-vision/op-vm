use napi::{
    bindgen_prelude::{BigInt, Promise},
    threadsafe_function::{ErrorStrategy, ThreadsafeFunction},
};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

#[napi(object)]
pub struct BlockHashRequest {
    pub block_number: BigInt,
    pub contract_id: BigInt,
}

#[napi(object)]
pub struct BlockHashResponse {
    #[napi(ts_type = "Buffer")]
    pub block_hash: Vec<u8>,
    pub is_block_warm: bool,
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

    pub fn execute(
        &self,
        block_number: u64,
        runtime: &Runtime,
    ) -> Result<BlockHashResponse, RuntimeError> {
        let request = BlockHashRequest {
            block_number: block_number.into(),
            contract_id: BigInt::from(self.contract_id),
        };

        let result = async move {
            let response: Result<Promise<BlockHashResponse>, RuntimeError> = self
                .tsfn
                .call_async(Ok(request))
                .await
                .map_err(|e| RuntimeError::new(e.reason));

            let promise = response?;

            let data = promise.await.map_err(|e| RuntimeError::new(e.reason))?;
            Ok(data)
        };

        let response = runtime.block_on(result);

        response
    }
}
