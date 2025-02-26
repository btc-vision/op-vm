use crate::domain::runner::BitcoinNetwork;

#[napi]
pub enum BitcoinNetworkRequest {
    Mainnet,
    Testnet,
    Regtest,
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
