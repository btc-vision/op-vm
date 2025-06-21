use crate::domain::runner::BitcoinNetwork;
use neon::context::Context;
use neon::prelude::JsNumber;
use neon::result::JsResult;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum BitcoinNetworkRequest {
    Mainnet = 0,
    Testnet = 1,
    Regtest = 2,
}

impl Into<BitcoinNetwork> for BitcoinNetworkRequest {
    fn into(self) -> BitcoinNetwork {
        match self {
            BitcoinNetworkRequest::Mainnet => BitcoinNetwork::Mainnet,
            BitcoinNetworkRequest::Testnet => BitcoinNetwork::Testnet,
            BitcoinNetworkRequest::Regtest => BitcoinNetwork::Regtest,
        }
    }
}

impl BitcoinNetworkRequest {
    #[allow(dead_code)]
    pub fn to_js_object<'a, C>(&self, cx: &'a mut C) -> JsResult<'a, JsNumber>
    where
        C: Context<'a>,
    {
        let value: u8 = *self as u8;
        Ok(cx.number(value))
    }
}

impl TryFrom<u8> for BitcoinNetworkRequest {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Mainnet),
            1 => Ok(Self::Testnet),
            2 => Ok(Self::Regtest),
            _ => Err(String::from("Unknown network")),
        }
    }
}
