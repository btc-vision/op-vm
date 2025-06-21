use crate::interfaces::napi::bitcoin_network_request::BitcoinNetworkRequest;
use bytes::Bytes;

pub struct JsContractParameter {
    pub(crate) bytecode: Option<Vec<u8>>,
    pub(crate) serialized: Option<Bytes>,
    pub(crate) used_gas: u64,
    pub(crate) max_gas: u64,
    pub(crate) memory_pages_used: u32,
    pub(crate) network: BitcoinNetworkRequest,
    pub(crate) is_debug_mode: bool,
    pub(crate) return_proofs: bool,
}
