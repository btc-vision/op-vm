use crate::domain::runner::ExitData;
use napi::bindgen_prelude::BigInt;

#[napi(object)]
pub struct ExitDataResponse {
    pub status: u32,
    #[napi(ts_type = "Buffer")]
    pub data: Vec<u8>,
    #[napi(ts_type = "bigint")]
    pub gas_used: BigInt,
    #[napi(ts_type = "Buffer[]")]
    pub proofs: Vec<Vec<u8>>,
}

impl From<ExitData> for ExitDataResponse {
    fn from(exit_data: ExitData) -> Self {
        ExitDataResponse {
            status: exit_data.status,
            data: exit_data.data,
            gas_used: BigInt::from(exit_data.gas_used),
            proofs: exit_data.proofs,
        }
    }
}
