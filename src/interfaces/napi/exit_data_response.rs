use crate::domain::runner::{ExitData, ProvenStateWrapped};
use napi::bindgen_prelude::BigInt;

#[cfg(feature = "use-strings-instead-of-buffers")]
use crate::domain::vm::vec_to_hex;

#[cfg(not(feature = "use-strings-instead-of-buffers"))]
#[napi(object)]
pub struct ExitDataResponse {
    pub status: u32,
    #[napi(ts_type = "Buffer")]
    pub data: Vec<u8>,
    #[napi(ts_type = "bigint")]
    pub gas_used: BigInt,
    #[napi(ts_type = "Array<{ proof: Buffer, vk: Buffer }>")]
    pub proofs: Vec<ProvenStateWrapped>,
}

#[cfg(feature = "use-strings-instead-of-buffers")]
#[napi(object)]
pub struct ExitDataResponse {
    pub status: u32,
    #[napi(ts_type = "string")]
    pub data: String,
    #[napi(ts_type = "bigint")]
    pub gas_used: BigInt,
    #[napi(ts_type = "Array<{ proof: string, vk: string }>")]
    pub proofs: Vec<ProvenStateWrapped>,
}

#[cfg(feature = "use-strings-instead-of-buffers")]
impl From<ExitData> for ExitDataResponse {
    fn from(exit_data: ExitData) -> Self {
        let proofs = exit_data
            .proofs
            .iter()
            .map(|p| ProvenStateWrapped {
                proof: vec_to_hex(&p.proof.clone()),
                vk: vec_to_hex(&p.vk.clone()),
            })
            .collect();

        ExitDataResponse {
            status: exit_data.status,
            data: vec_to_hex(&exit_data.data),
            gas_used: BigInt::from(exit_data.gas_used),
            proofs,
        }
    }
}

#[cfg(not(feature = "use-strings-instead-of-buffers"))]
impl From<ExitData> for ExitDataResponse {
    fn from(exit_data: ExitData) -> Self {
        let proofs = exit_data
            .proofs
            .iter()
            .map(|p| ProvenStateWrapped {
                proof: p.proof.clone(),
                vk: p.vk.clone(),
            })
            .collect();

        ExitDataResponse {
            status: exit_data.status,
            data: exit_data.data,
            gas_used: BigInt::from(exit_data.gas_used),
            proofs,
        }
    }
}
