use std::sync::Arc;

use neon::{prelude::*, types::JsBigInt};

use super::AsArguments;
use crate::interfaces::ExternalFunction;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub struct GenericExternalFunction {
    channel: Channel,
    handle: Arc<Root<JsFunction>>,
    contract_id: u64,
}

pub struct EmptyFunctionRequest {
    contract_id: u64,
}

impl AsArguments for EmptyFunctionRequest {
    fn as_arguments<'a, C>(
        &self,
        cx: &mut C,
    ) -> neon::prelude::NeonResult<Vec<Handle<'a, neon::prelude::JsValue>>>
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
    fn as_arguments<'a, C>(
        &self,
        cx: &mut C,
    ) -> neon::prelude::NeonResult<Vec<Handle<'a, neon::prelude::JsValue>>>
    where
        C: Context<'a>,
    {
        Ok(vec![
            JsBigInt::from_u64(cx, self.contract_id).upcast(),
            JsBuffer::from_slice(cx, &self.buffer)?.upcast(),
        ])
    }
}

impl ExternalFunction<Vec<u8>> for GenericExternalFunction {
    fn handle(&self) -> Arc<Root<JsFunction>> {
        self.handle.clone()
    }

    fn channel(&self) -> Channel {
        self.channel.clone()
    }
}

impl ExternalFunction<()> for GenericExternalFunction {
    fn handle(&self) -> std::sync::Arc<Root<JsFunction>> {
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

    pub fn execute(&self, call_data: &[u8], runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let args = BufferFunctionRequest {
            buffer: call_data.to_vec(),
            contract_id: self.contract_id,
        };

        Ok(self
            .call(runtime, args)
            .or_else(|err| Err(RuntimeError::new(err.to_string())))?)
    }

    pub fn execute_empty_request(&self, runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let args = EmptyFunctionRequest {
            contract_id: self.contract_id,
        };

        Ok(self
            .call(runtime, args)
            .or_else(|err| Err(RuntimeError::new(err.to_string())))?)
    }

    pub fn execute_no_response(
        &self,
        call_data: &[u8],
        runtime: &Runtime,
    ) -> Result<(), RuntimeError> {
        let args = BufferFunctionRequest {
            buffer: call_data.to_vec(),
            contract_id: self.contract_id,
        };

        Ok(self
            .call(runtime, args)
            .or_else(|err| Err(RuntimeError::new(err.to_string())))?)
    }
}
