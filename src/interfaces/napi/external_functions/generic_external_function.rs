use std::sync::Arc;

use neon::{
    event::Channel,
    prelude::*,
    types::{JsBigInt, JsBuffer},
};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use super::AsArguments;
use crate::interfaces::ExternalFunction;

pub struct EmptyFunctionRequest {
    contract_id: u64,
}

impl AsArguments for EmptyFunctionRequest {
    fn as_arguments<'a, C>(&self, cx: &mut C) -> NeonResult<Vec<Handle<'a, JsValue>>>
    where
        C: Context<'a>,
    {
        Ok(vec![JsBigInt::from_u64(cx, self.contract_id).upcast()])
    }
}

pub struct BufferFunctionRequest {
    pub contract_id: u64,
    pub buffer: Vec<u8>,
}

impl AsArguments for BufferFunctionRequest {
    fn as_arguments<'a, C>(&self, cx: &mut C) -> NeonResult<Vec<Handle<'a, JsValue>>>
    where
        C: Context<'a>,
    {
        let object = cx.empty_object();
        let object_buffer = JsBuffer::from_slice(cx, &self.buffer)?;
        let object_contract_id = JsBigInt::from_u64(cx, self.contract_id);
        object.set(cx, "buffer", object_buffer)?;
        object.set(cx, "contractId", object_contract_id)?;

        Ok(vec![object.upcast()])
    }
}

#[derive(Clone)]
pub struct GenericExternalFunction {
    channel: Channel,
    handle: Arc<Root<JsFunction>>,
    contract_id: u64,
}

/* one impl per JS return type */
impl ExternalFunction<Vec<u8>> for GenericExternalFunction {
    fn handle(&self) -> Arc<Root<JsFunction>> {
        self.handle.clone()
    }
    fn channel(&self) -> Channel {
        self.channel.clone()
    }
}

impl ExternalFunction<()> for GenericExternalFunction {
    fn handle(&self) -> Arc<Root<JsFunction>> {
        self.handle.clone()
    }
    fn channel(&self) -> Channel {
        self.channel.clone()
    }
}

impl GenericExternalFunction {
    pub fn new(handle: Arc<Root<JsFunction>>, channel: Channel, contract_id: u64) -> Self {
        Self {
            handle,
            channel,
            contract_id,
        }
    }

    /// Call the JS function, expecting **bytes** back.
    pub fn execute(&self, call_data: &[u8], rt: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let args = BufferFunctionRequest {
            buffer: call_data.to_vec(),
            contract_id: self.contract_id,
        };
        // pick the Vec<u8> blanket impl explicitly
        ExternalFunction::<Vec<u8>>::call_blocking(self, rt, args)
    }

    /// Same, but with no call-data (`EmptyFunctionRequest`).
    pub fn execute_empty_request(&self, rt: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let args = EmptyFunctionRequest {
            contract_id: self.contract_id,
        };
        ExternalFunction::<Vec<u8>>::call_blocking(self, rt, args)
    }

    /// Fire-and-forget variant: JS returns `undefined` / `()`.
    pub fn execute_no_response(&self, call_data: &[u8], rt: &Runtime) -> Result<(), RuntimeError> {
        let args = BufferFunctionRequest {
            buffer: call_data.to_vec(),
            contract_id: self.contract_id,
        };
        ExternalFunction::<()>::call_blocking(self, rt, args)
    }
}
