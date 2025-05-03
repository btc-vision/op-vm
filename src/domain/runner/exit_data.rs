use bitcoin::hex::DisplayHex;
use core::num;
use neon::{prelude::*, types::JsBigInt};
use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct ExitData {
    pub status: u32,
    pub data: Vec<u8>,
    pub gas_used: u64,
}

impl ExitData {
    pub fn new(status: u32, gas_used: u64, data: &[u8]) -> Self {
        Self {
            status,
            data: data.to_vec(),
            gas_used,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.status == 0
    }

    pub fn to_js_object<'a, C>(&self, cx: &mut C) -> JsResult<'a, JsObject>
    where
        C: Context<'a>,
    {
        let object = cx.empty_object();
        let number = cx.number(self.status);
        let data = JsBuffer::from_slice(cx, &self.data)?;
        let gas_used = JsBigInt::from_u64(cx, self.gas_used);

        object.set(cx, "status", number)?;
        object.set(cx, "data", data)?;
        object.set(cx, "gasUSed", gas_used)?;

        Ok(object)
    }
}

impl Finalize for ExitData {}

impl Display for ExitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "status: {}, gas_used: {}, data: {}",
            self.status,
            self.gas_used,
            self.data.to_lower_hex_string(),
        )
    }
}
