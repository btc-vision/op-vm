use crate::domain::runner::Network;

#[napi]
pub enum NetworkRequest {
    Mainnet,
    Testnet,
    Regtest,
}

impl Into<Network> for NetworkRequest {
    fn into(self) -> Network {
        match self {
            NetworkRequest::Mainnet => Network::Mainnet,
            NetworkRequest::Testnet => Network::Testnet,
            NetworkRequest::Regtest => Network::Regtest,
        }
    }
}