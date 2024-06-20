use napi::bindgen_prelude::{Array, BigInt};

#[napi(object)]
pub struct CallResponse {
    pub result: Array,
    pub gas_used: BigInt,
}
