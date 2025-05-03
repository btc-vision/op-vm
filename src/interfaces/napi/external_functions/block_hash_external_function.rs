use std::sync::Arc;

use neon::{prelude::*, types::buffer::TypedArray};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use super::{AsArguments, ExternalFunction, FromJsObject};

pub struct BlockHashRequest {
    pub block_number: u64,
    pub contract_id: u64,
}

impl AsArguments for BlockHashRequest {
    fn as_arguments<'a, C>(
        &self,
        cx: &mut C,
    ) -> neon::prelude::NeonResult<Vec<Handle<'a, neon::prelude::JsValue>>>
    where
        C: Context<'a>,
    {
        Ok(vec![cx.undefined().upcast()])
    }
}

pub struct BlockHashResponse {
    pub block_hash: Vec<u8>,
    pub is_block_warm: bool,
}

impl FromJsObject for BlockHashResponse {
    fn from_js_object<'a, C>(cx: &mut C, obj: Handle<'a, JsValue>) -> anyhow::Result<Self>
    where
        C: Context<'a>,
    {
        let obj = obj
            .downcast_or_throw::<JsObject, _>(cx)
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;

        Ok(BlockHashResponse {
            block_hash: obj
                .get::<JsBuffer, _, _>(cx, "blockHash")
                .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?
                .as_slice(cx)
                .to_vec(),
            is_block_warm: obj
                .get::<JsBoolean, _, _>(cx, "isBlockWarm")
                .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?
                .value(cx),
        })
    }
}

pub struct BlockHashExternalFunction {
    handle: Arc<Root<JsFunction>>,
    channel: Channel,
    contract_id: u64,
}

impl ExternalFunction<BlockHashResponse> for BlockHashExternalFunction {
    fn handle(&self) -> std::sync::Arc<Root<JsFunction>> {
        self.handle.clone()
    }

    fn channel(&self) -> Channel {
        self.channel.clone()
    }
}

impl BlockHashExternalFunction {
    pub fn new(handle: Arc<Root<JsFunction>>, channel: Channel, contract_id: u64) -> Self {
        Self {
            handle,
            channel,
            contract_id,
        }
    }

    pub fn execute(
        &self,
        block_number: u64,
        runtime: &Runtime,
    ) -> Result<BlockHashResponse, RuntimeError> {
        let args = BlockHashRequest {
            block_number: block_number,
            contract_id: self.contract_id,
        };

        Ok(self
            .call(runtime, args)
            .or_else(|err| Err(RuntimeError::new(err.to_string())))?)
    }
}
