use crate::interfaces::napi::bitcoin_network_request::BitcoinNetworkRequest;
use bytes::Bytes;
use napi::JsFunction;

pub struct JsContractParameter {
    pub(crate) bytecode: Option<Vec<u8>>,
    pub(crate) serialized: Option<Bytes>,
    pub(crate) max_gas: u64,
    pub(crate) network: BitcoinNetworkRequest,
    pub(crate) storage_load_js_function: JsFunction,
    pub(crate) storage_store_js_function: JsFunction,
    pub(crate) call_other_contract_js_function: JsFunction,
    pub(crate) deploy_from_address_js_function: JsFunction,
    pub(crate) console_log_js_function: JsFunction,
}
