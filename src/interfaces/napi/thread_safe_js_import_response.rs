use napi::bindgen_prelude::BigInt;

#[cfg(not(feature = "use-strings-instead-of-buffers"))]
#[napi(object)]
pub struct ThreadSafeJsImportResponse {
    pub buffer: Vec<u8>,
    pub contract_id: BigInt,
}

#[cfg(feature = "use-strings-instead-of-buffers")]
#[napi(object)]
pub struct ThreadSafeJsImportResponse {
    pub buffer: String,
    pub contract_id: BigInt,
}
