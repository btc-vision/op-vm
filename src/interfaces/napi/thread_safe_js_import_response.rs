use napi::bindgen_prelude::BigInt;

#[napi(object)]
pub struct ThreadSafeJsImportResponse {
    pub buffer: Vec<u8>,
    pub contract_id: BigInt,
}
