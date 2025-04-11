use crate::domain::common::Address;
use crate::domain::runner::EnvironmentVariables;
use neon::object::PropOptions;
use neon::prelude::*;
use neon::types::bigint::RangeError;
use neon::types::JsBigInt;

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
        let mut block_hash: PropOptions<JsObject, &str> = obj.prop(cx, "blockHash");
        let mut block_hash_value = block_hash.get::<Vec<u8>>();

        // block_number -> JsBigInt -> u64
        let mut block_number: PropOptions<JsObject, &str> = obj.prop(cx, "blockNumber");
        let block_number_val: u128 = block_number.get::<u128>()?;

        // block_median_time -> JsBigInt -> u64
        let mut block_median_time: PropOptions<JsObject, &str> = obj.prop(cx, "blockMedianTime");
        let block_median_time_val = block_median_time.get::<u64>()?;

        // tx_id -> JsBuffer -> Vec<u8>
        let mut tx_id: PropOptions<JsObject, &str> = obj.prop(cx, "txId");
        let mut tx_id_value = tx_id.get::<Vec<u8>>();

        // tx_hash -> JsBuffer -> Vec<u8>
        let mut tx_hash: PropOptions<JsObject, &str> = obj.prop(cx, "txHash");
        let mut tx_hash_value = tx_hash.get::<Vec<u8>>();

        // contract_address -> JsBuffer -> Vec<u8>
        let mut contract_address: PropOptions<JsObject, &str> = obj.prop(cx, "contractAddress");
        let mut contract_address_value = contract_address.get::<Vec<u8>>();

        // contract_deployer -> JsBuffer -> Vec<u8>
        let mut contract_deployer: PropOptions<JsObject, &str> = obj.prop(cx, "contractDeployer");
        let mut contract_deployer_value = contract_deployer.get::<Vec<u8>>();

        // caller -> JsBuffer -> Vec<u8>
        let mut caller: PropOptions<JsObject, &str> = obj.prop(cx, "caller");
        let mut caller_value = caller.get::<Vec<u8>>();

        // origin -> JsBuffer -> Vec<u8>
        let mut origin: PropOptions<JsObject, &str> = obj.prop(cx, "origin");
        let mut origin_value = origin.get::<Vec<u8>>();

        // Convert JsBuffer to Vec<u8>
        let block_hash = block_hash_value
            .as_ref()
            .map(|buffer| buffer.to_vec())
            .unwrap_or_else(|_| {
                cx.throw_type_error("block_hash is required").unwrap();
                vec![]
            });

        let tx_id = tx_id_value
            .as_ref()
            .map(|buffer| buffer.to_vec())
            .unwrap_or_else(|_| {
                cx.throw_type_error("tx_id is required").unwrap();
                vec![]
            });

        let tx_hash = tx_hash_value
            .as_ref()
            .map(|buffer| buffer.to_vec())
            .unwrap_or_else(|_| {
                cx.throw_type_error("tx_hash is required").unwrap();
                vec![]
            });

        let contract_address = contract_address_value
            .as_ref()
            .map(|buffer| buffer.to_vec())
            .unwrap_or_else(|_| {
                cx.throw_type_error("contract_address is required").unwrap();
                vec![]
            });

        let contract_deployer = contract_deployer_value
            .as_ref()
            .map(|buffer| buffer.to_vec())
            .unwrap_or_else(|_| {
                cx.throw_type_error("contract_deployer is required")
                    .unwrap();
                vec![]
            });

        let caller = caller_value
            .as_ref()
            .map(|buffer| buffer.to_vec())
            .unwrap_or_else(|_| {
                cx.throw_type_error("caller is required").unwrap();
                vec![]
            });

        let origin = origin_value
            .as_ref()
            .map(|buffer| buffer.to_vec())
            .unwrap_or_else(|_| {
                cx.throw_type_error("origin is required").unwrap();
                vec![]
            });

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
        )
    }
}

/// Example Neon function that expects one argument (an object) and returns undefined.
/// In real usage, you might return some computed result or store `env_vars` somewhere.
pub fn create_environment_variables(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let id = cx
        .argument::<JsBigInt>(0)?
        .to_u64(&mut cx)
        .unwrap_or_else(|e| {
            cx.throw_range_error::<String, RangeError<u64>>(format!("{:?}", e))
                .unwrap();
            0
        });

    let obj = cx.argument::<JsObject>(1)?;

    // 2. Parse it into our Rust struct
    let req = EnvironmentVariablesRequest::from_js_object(&mut cx, obj)?;

    // 3. Convert to your domain type
    let env_vars: EnvironmentVariables = req.into();
    // ... do something with `env_vars` here ...

    // Return `undefined` (or something else) to JavaScript
    Ok(cx.undefined())
}
