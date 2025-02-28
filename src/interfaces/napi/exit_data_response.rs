use crate::domain::runner::ExitData;

#[napi(object)]
pub struct ExitDataResponse {
    pub status: u32,
    #[napi(ts_type = "Buffer")]
    pub data: Vec<u8>,
}

impl From<ExitData> for ExitDataResponse {
    fn from(exit_data: ExitData) -> Self {
        ExitDataResponse {
            status: exit_data.status,
            data: exit_data.data,
        }
    }
}
