use napi::bindgen_prelude::{Array, BigInt};

#[napi(object)]
pub struct CallResponse {
    #[napi(ts_type = "number[]")]
    pub result: Array,
    pub gas_used: BigInt,
}
