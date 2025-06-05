use crate::domain::runner::ProvenState;
use bitcoin::hex::DisplayHex;
use napi::bindgen_prelude::{BufferSlice, ToNapiValue};
use napi::{
    bindgen_prelude::{FromNapiValue, JsObjectValue, Object},
    sys, Env,
};
use napi::{JsValue, Result as NapiResult};
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
    unsafe fn to_napi_value(
        env_raw: napi::sys::napi_env,
        mut val: Self,
    ) -> napi::Result<napi::sys::napi_value> {
        let env = Env::from_raw(env_raw);

        let mut obj = Object::new(&env)?;

        obj.set_named_property("status", val.status)?;
        obj.set_named_property(
            "data",
            BufferSlice::copy_from(&env, std::mem::take(&mut val.data))?,
        )?;
        obj.set_named_property("gasUsed", val.gas_used)?;

        let mut arr = env.create_array(val.proofs.len() as u32)?;
        for proof in &mut val.proofs {
            let mut p_obj = Object::new(&env)?;
            p_obj.set_named_property(
                "proof",
                BufferSlice::copy_from(&env, std::mem::take(&mut proof.proof))?,
            )?;
            p_obj.set_named_property(
                "vk",
                BufferSlice::copy_from(&env, std::mem::take(&mut proof.vk))?,
            )?;
            arr.insert(p_obj)?;
        }
        obj.set_named_property("proofs", arr)?;

        Ok(obj.raw())
    }
}

impl FromNapiValue for ExitData {
    unsafe fn from_napi_value(env_raw: sys::napi_env, value: sys::napi_value) -> NapiResult<Self> {
        let obj = Object::from_raw(env_raw, value);
        let proofs: Vec<Object> = obj.get_named_property("proofs")?;
        Ok(ExitData::new(
            obj.get_named_property("status")?,
            obj.get_named_property::<u32>("gasUsed")? as u64,
            obj.get_named_property("data")?,
            proofs
                .into_iter()
                .map(|p| {
                    Ok(ProvenState {
                        proof: p.get_named_property::<BufferSlice>("proof")?.to_vec(),
                        vk: p.get_named_property::<BufferSlice>("vk")?.to_vec(),
                    })
                })
                .collect::<NapiResult<Vec<_>>>()?,
        ))
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
