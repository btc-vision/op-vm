use crate::interfaces::napi::bitcoin_network_request::BitcoinNetworkRequest;
use bytes::Bytes;

pub struct JsContractParameter {
    pub(crate) bytecode: Option<Vec<u8>>,
    pub(crate) serialized: Option<Bytes>,
    pub(crate) max_gas: u64,
    pub(crate) network: BitcoinNetworkRequest,
}
