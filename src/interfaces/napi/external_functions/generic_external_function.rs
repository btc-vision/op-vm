use std::sync::Arc;

use neon::{prelude::*, types::JsBigInt};
use sha2::digest::typenum::TypeArray;

use super::AsArguments;
use crate::interfaces::ExternalFunction;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub struct GenericExternalFunction {
    name: String,
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
        let object = cx.empty_object();
        let object_buffer = JsTypedArray::from_slice(cx, &self.buffer)?;
        let object_contract_id = JsBigInt::from_u64(cx, self.contract_id);
        object.set(cx, "buffer", object_buffer)?;
        object.set(cx, "contractId", object_contract_id)?;

        let console = cx.global_object().get::<JsObject, _, _>(cx, "console")?;
        let log = console.get::<JsFunction, _, _>(cx, "log")?;

        let args = vec![cx.undefined().upcast(), object.upcast()];
        log.call(cx, console, args.clone()).unwrap();
        Ok(args)
    }
}

impl ExternalFunction<Vec<u8>> for GenericExternalFunction {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle(&self) -> Arc<Root<JsFunction>> {
        self.handle.clone()
    }

    fn channel(&self) -> Channel {
        self.channel.clone()
    }
}

impl ExternalFunction<()> for GenericExternalFunction {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle(&self) -> std::sync::Arc<Root<JsFunction>> {
        self.handle.clone()
    }

    fn channel(&self) -> Channel {
        self.channel.clone()
    }
}

impl GenericExternalFunction {
    pub fn new(
        name: &str,
        handle: Arc<Root<JsFunction>>,
        channel: Channel,
        contract_id: u64,
    ) -> Self {
        Self {
            name: name.to_string(),
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

        self.call(runtime, args)
    }

    pub fn execute_empty_request(&self, runtime: &Runtime) -> Result<Vec<u8>, RuntimeError> {
        let args = EmptyFunctionRequest {
            contract_id: self.contract_id,
        };

        self.call(runtime, args)
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

        self.call(runtime, args)
    }
}
