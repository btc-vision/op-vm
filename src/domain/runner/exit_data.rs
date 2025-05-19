use crate::domain::runner::ProvenState;
use bitcoin::hex::DisplayHex;
use napi::bindgen_prelude::{BufferSlice, ToNapiValue};
use napi::Result as NapiResult;
use napi::{sys, Env, NapiRaw};
use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct ExitData {
    pub status: u32,
    pub data: Vec<u8>,
    pub gas_used: u64,

    pub proofs: Vec<ProvenState>,
}

impl ExitData {
    pub fn new(status: u32, gas_used: u64, data: &[u8], proofs: Vec<ProvenState>) -> Self {
        Self {
            status,
            data: data.to_vec(),
            gas_used,
            proofs,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.status == 0
    }
}

impl ToNapiValue for ExitData {
    unsafe fn to_napi_value(env_raw: sys::napi_env, val: Self) -> NapiResult<sys::napi_value> {
        println!("ExitData::to_napi_value");

        let env = Env::from_raw(env_raw);

        let mut obj = env.create_object()?;
        obj.set_named_property("status", env.create_uint32(val.status))?;
        obj.set_named_property("data", BufferSlice::from_data(&env, val.data)?)?;
        obj.set_named_property("gasUsed", env.create_bigint_from_u64(val.gas_used))?;

        let mut arr = env.create_array(val.proofs.len() as u32)?;
        for (idx, p) in val.proofs.into_iter().enumerate() {
            let mut proof_obj = env.create_object()?;
            proof_obj.set_named_property("proof", BufferSlice::from_data(&env, p.proof)?)?;
            proof_obj.set_named_property("vk", BufferSlice::from_data(&env, p.vk)?)?;

            arr.insert(idx as u32)?;
        }

        obj.set_named_property("proofs", arr)?;

        Ok(obj.raw())
    }
}

impl Display for ExitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "status: {}, gas_used: {}, data: {}, proofs: {}",
            self.status,
            self.gas_used,
            self.data.to_lower_hex_string(),
            self.proofs
                .iter()
                .map(|p| format!(
                    "{{proof: {}, vk: {}}}",
                    p.proof.to_lower_hex_string(),
                    p.vk.to_lower_hex_string()
                ))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
