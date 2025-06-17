use neon::{
    prelude::*,
    types::{buffer::TypedArray, JsBigInt},
};

use crate::domain::{common::Address, runner::EnvironmentVariables};
pub struct EnvironmentVariablesRequest {
    pub block_hash: Vec<u8>,
    pub block_number: u64,
    pub block_median_time: u64,
    pub tx_id: Vec<u8>,
    pub tx_hash: Vec<u8>,
    pub contract_address: Vec<u8>,
    pub contract_deployer: Vec<u8>,
    pub caller: Vec<u8>,
    pub origin: Vec<u8>,
    pub chain_id: Vec<u8>,
    pub protocol_id: Vec<u8>,
}

impl EnvironmentVariablesRequest {
    /// Construct from a JavaScript object using Neon’s APIs.
    pub fn from_js_object(cx: &mut FunctionContext, obj: Handle<JsObject>) -> NeonResult<Self> {
        Ok(Self {
            block_hash: obj
                .get::<JsBuffer, _, _>(cx, "blockHash")?
                .as_slice(cx)
                .to_vec(),
            block_number: obj
                .get::<JsBigInt, _, _>(cx, "blockNumber")?
                .to_u64(cx)
                .or_else(|err| cx.throw_range_error(err.to_string()))?,
            block_median_time: obj
                .get::<JsBigInt, _, _>(cx, "blockMedianTime")?
                .to_u64(cx)
                .or_else(|err| cx.throw_range_error(err.to_string()))?,
            tx_id: obj.get::<JsBuffer, _, _>(cx, "txId")?.as_slice(cx).to_vec(),
            tx_hash: obj
                .get::<JsBuffer, _, _>(cx, "txHash")?
                .as_slice(cx)
                .to_vec(),
            contract_address: obj
                .get::<JsBuffer, _, _>(cx, "contractAddress")?
                .as_slice(cx)
                .to_vec(),
            contract_deployer: obj
                .get::<JsBuffer, _, _>(cx, "contractDeployer")?
                .as_slice(cx)
                .to_vec(),
            caller: obj
                .get::<JsBuffer, _, _>(cx, "caller")?
                .as_slice(cx)
                .to_vec(),
            origin: obj
                .get::<JsBuffer, _, _>(cx, "origin")?
                .as_slice(cx)
                .to_vec(),
            chain_id: obj
                .get::<JsBuffer, _, _>(cx, "chainId")?
                .as_slice(cx)
                .to_vec(),
            protocol_id: obj
                .get::<JsBuffer, _, _>(cx, "protocolId")?
                .as_slice(cx)
                .to_vec(),
        })
    }
}

/// Implement conversion from `EnvironmentVariablesRequest` to your
/// domain type `EnvironmentVariables`.

impl Into<EnvironmentVariables> for EnvironmentVariablesRequest {
    fn into(self) -> EnvironmentVariables {
        EnvironmentVariables::new(
            &self.block_hash,
            self.block_number,
            self.block_median_time,
            &self.tx_id,
            &self.tx_hash,
            Address::new(&self.contract_address),
            Address::new(&self.contract_deployer),
            Address::new(&self.caller),
            Address::new(&self.origin),
            &self.chain_id,
            &self.protocol_id,
        )
    }
}

/*pub fn create_environment_variables(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    // 1. Get the first argument as an object
    let obj = cx.argument::<JsObject>(0)?;

    // 2. Parse it into our Rust struct
    let req = EnvironmentVariablesRequest::from_js_object(&mut cx, obj)?;

    // 3. Convert to your domain type
    let env_vars: EnvironmentVariables = req.into();
    // ... do something with `env_vars` here ...

    // Return `undefined` (or something else) to JavaScript
    Ok(cx.undefined())
}*/
