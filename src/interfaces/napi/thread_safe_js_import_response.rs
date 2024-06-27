#[napi(object)]
pub struct ThreadSafeJsImportResponse {
    pub buffer: Vec<u8>,
}
