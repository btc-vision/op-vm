use napi::{
    bindgen_prelude::{BigInt, Promise},
    threadsafe_function::ThreadsafeFunction,
};
use std::sync::Arc;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

#[cfg(feature = "use-strings-instead-of-buffers")]
use crate::domain::vm::hex_to_vec;

#[cfg(not(feature = "use-strings-instead-of-buffers"))]
use napi::bindgen_prelude::Uint8Array;

#[napi(object)]
pub struct BlockHashRequest {
    pub block_number: BigInt,
    pub contract_id: BigInt,
}

#[cfg(not(feature = "use-strings-instead-of-buffers"))]
#[napi(object, js_name = "BlockHashResponse")]
pub struct JsBlockHashResponse {
    pub block_hash: Uint8Array,
    pub is_block_warm: bool,
}

#[cfg(feature = "use-strings-instead-of-buffers")]
#[napi(object, js_name = "BlockHashResponse")]
pub struct JsBlockHashResponse {
    pub block_hash: String,
    pub is_block_warm: bool,
}

pub struct BlockHashResponse {
    pub block_hash: Vec<u8>,
    pub is_block_warm: bool,
}

pub struct BlockHashExternalFunction {
    tsfn: Arc<
        ThreadsafeFunction<
            BlockHashRequest,
            Promise<JsBlockHashResponse>,
            BlockHashRequest,
            true,
            true,
            128,
        >,
    >,
    contract_id: u64,
}

impl BlockHashExternalFunction {
    pub fn new(
        tsfn: Arc<
            ThreadsafeFunction<
                BlockHashRequest,
                Promise<JsBlockHashResponse>,
                BlockHashRequest,
                true,
                true,
                128,
            >,
        >,
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
            let response: Result<Promise<JsBlockHashResponse>, RuntimeError> = self
                .tsfn
                .call_async(Ok(request))
                .await
                .map_err(|e| RuntimeError::new(e.reason.clone()));

            let promise = response?;

            let data = promise
                .await
                .map_err(|e| RuntimeError::new(e.reason.clone()))?;

            #[cfg(feature = "use-strings-instead-of-buffers")]
            let block_hash = hex_to_vec(data.block_hash)?;

            #[cfg(not(feature = "use-strings-instead-of-buffers"))]
            let block_hash = data.block_hash;

            Ok(BlockHashResponse {
                block_hash: block_hash.to_vec(),
                is_block_warm: data.is_block_warm,
            })
        };

        let response = runtime.block_on(result);

        response
    }
}
