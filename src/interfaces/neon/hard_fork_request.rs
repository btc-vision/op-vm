use neon::context::Context;
use neon::prelude::{JsNumber, JsResult};
use crate::domain::runner::HardFork;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum HardForkRequest {
    Roswell = 0,
    Rachel = 1,
}

impl Into<HardFork> for HardForkRequest {
    fn into(self) -> HardFork {
        match self {
            HardForkRequest::Roswell => HardFork::Roswell,
            HardForkRequest::Rachel => HardFork::Rachel,
        }
    }
}

impl HardForkRequest {
    #[allow(dead_code)]
    pub fn to_js_object<'a, C>(&self, cx: &'a mut C) -> JsResult<'a, JsNumber>
    where
        C: Context<'a>,
    {
        let value: u8 = *self as u8;
        Ok(cx.number(value))
    }
}


impl TryFrom<u8> for HardForkRequest {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HardForkRequest::Roswell),
            1 => Ok(HardForkRequest::Rachel),
            _ => Err(String::from("Unknown hard fork")),
        }
    }
}
