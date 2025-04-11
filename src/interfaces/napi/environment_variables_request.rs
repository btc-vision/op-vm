use crate::domain::common::Address;
use crate::domain::runner::EnvironmentVariables;
use neon::prelude::*;
use neon::types::buffer::TypedArray;

/// Plain Rust struct to hold the request data
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
}

impl EnvironmentVariablesRequest {
    /// Construct from a JavaScript object using Neon’s APIs.
    pub fn from_js_object(cx: &mut FunctionContext, obj: Handle<JsObject>) -> NeonResult<Self> {
        // block_hash -> JsBuffer -> Vec<u8>
        let block_hash = obj.get(cx, "block_hash")?;
        let mut block_hash = block_hash.downcast_or_throw::<JsBuffer, _>(cx)?;

        let test = block_hash.as_mut_slice(cx).to_vec();

        println!("test: {:?}", test);

        panic!("test");

        // block_number -> JsBigInt -> (u64, bool)
        // JsBigInt::u64(cx) returns (u64_value, sign_bit)
        /*let block_number_js = obj.get(cx, "block_number")?;
        let block_number_js = block_number_js.downcast_or_throw::<JsBigInt, _>(cx)?;
        let (block_number_val, _block_number_sign) = block_number_js.to_u64(cx)?;

        // block_median_time -> JsBigInt
        let block_median_time_js = obj.get(cx, "block_median_time")?;
        let block_median_time_js = block_median_time_js.downcast_or_throw::<JsBigInt, _>(cx)?;
        let (block_median_time_val, _block_median_time_sign) = block_median_time_js.to_u64(cx)?;

        // tx_id -> JsBuffer -> Vec<u8>
        let tx_id = obj.get(cx, "tx_id")?;
        let tx_id = tx_id.downcast_or_throw::<JsBuffer, _>(cx)?.to_vec(cx)?;

        // tx_hash -> JsBuffer -> Vec<u8>
        let tx_hash = obj.get(cx, "tx_hash")?;
        let tx_hash = tx_hash.downcast_or_throw::<JsBuffer, _>(cx)?.to_vec(cx)?;

        // contract_address -> JsBuffer -> Vec<u8>
        let contract_address = obj.get(cx, "contract_address")?;
        let contract_address = contract_address
            .downcast_or_throw::<JsBuffer, _>(cx)?
            .to_vec(cx)?;

        // contract_deployer -> JsBuffer -> Vec<u8>
        let contract_deployer = obj.get(cx, "contract_deployer")?;
        let contract_deployer = contract_deployer
            .downcast_or_throw::<JsBuffer, _>(cx)?
            .to_vec(cx)?;

        // caller -> JsBuffer -> Vec<u8>
        let caller = obj.get(cx, "caller")?;
        let caller = caller.downcast_or_throw::<JsBuffer, _>(cx)?.to_vec(cx)?;

        // origin -> JsBuffer -> Vec<u8>
        let origin = obj.get(cx, "origin")?;
        let origin = origin.downcast_or_throw::<JsBuffer, _>(cx)?.to_vec(cx)?;

        Ok(EnvironmentVariablesRequest {
            block_hash,
            block_number: block_number_val,
            block_median_time: block_median_time_val,
            tx_id,
            tx_hash,
            contract_address,
            contract_deployer,
            caller,
            origin,
        })*/
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
        )
    }
}

/// Example Neon function that expects one argument (an object) and returns undefined.
/// In real usage, you might return some computed result or store `env_vars` somewhere.
pub fn create_environment_variables(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    // 1. Get the first argument as an object
    let obj = cx.argument::<JsObject>(0)?;

    // 2. Parse it into our Rust struct
    let req = EnvironmentVariablesRequest::from_js_object(&mut cx, obj)?;

    // 3. Convert to your domain type
    let env_vars: EnvironmentVariables = req.into();
    // ... do something with `env_vars` here ...

    // Return `undefined` (or something else) to JavaScript
    Ok(cx.undefined())
}

/// Register the Neon module and export the function
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("", create_environment_variables)?;
    Ok(())
}
