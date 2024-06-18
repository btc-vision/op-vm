use crate::domain::contract::AbortData;

#[napi(object)]
pub struct AbortDataResponse {
    pub message: u32,
    pub file_name: u32,
    pub line: u32,
    pub column: u32,
}

impl From<AbortData> for AbortDataResponse {
    fn from(data: AbortData) -> Self {
        AbortDataResponse {
            message: data.message,
            file_name: data.file_name,
            line: data.line,
            column: data.column,
        }
    }
}
