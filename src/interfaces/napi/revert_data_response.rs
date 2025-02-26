use crate::domain::runner::RevertData;

#[napi(object)]
pub struct RevertDataResponse {
    pub data: Vec<u8>,
}

impl From<RevertData> for RevertDataResponse {
    fn from(revert_data: RevertData) -> Self {
        RevertDataResponse {
            data: revert_data.data,
        }
    }
}
