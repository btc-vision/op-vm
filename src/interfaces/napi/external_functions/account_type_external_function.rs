use std::sync::Arc;

use neon::prelude::*;
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

use super::{BufferFunctionRequest, ExternalFunction, FromJsObject};

pub struct AccountTypeResponse {
    pub account_type: u32,
    pub is_address_warm: bool,
}

impl FromJsObject for AccountTypeResponse {
    fn from_js_object<'a, C>(cx: &mut C, obj: Handle<'a, JsValue>) -> anyhow::Result<Self>
    where
        C: Context<'a>,
    {
        let obj = obj
            .downcast_or_throw::<JsObject, _>(cx)
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;

        Ok(AccountTypeResponse {
            account_type: obj
                .get::<JsNumber, _, _>(cx, "accountType")
                .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?
                .value(cx) as u32,
            is_address_warm: obj
                .get::<JsBoolean, _, _>(cx, "isAddressWarm")
                .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?
                .value(cx),
        })
    }
}

pub struct AccountTypeExternalFunction {
    handle: Arc<Root<JsFunction>>,
    channel: Channel,
    contract_id: u64,
}

impl AccountTypeExternalFunction {
    pub fn new(handle: Arc<Root<JsFunction>>, channel: Channel, contract_id: u64) -> Self {
        Self {
            handle,
            channel,
            contract_id,
        }
    }
}

impl super::external_function::ExternalFunction<AccountTypeResponse>
    for AccountTypeExternalFunction
{
    fn handle(&self) -> Arc<Root<JsFunction>> {
        self.handle.clone()
    }

    fn channel(&self) -> Channel {
        self.channel.clone()
    }
}

impl AccountTypeExternalFunction {
    pub fn execute(
        &self,
        address_hash: &[u8],
        runtime: &Runtime,
    ) -> Result<AccountTypeResponse, RuntimeError> {
        let args = BufferFunctionRequest {
            buffer: address_hash.to_vec(),
            contract_id: self.contract_id,
        };

        Ok(self
            .call(runtime, args)
            .or_else(|err| Err(RuntimeError::new(err.to_string())))?)
    }
}
