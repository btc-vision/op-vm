pub enum BitcoinNetwork {
    Mainnet,
    Testnet,
    Regtest,
}

impl BitcoinNetwork {
    pub fn contract_address_prefix(&self) -> String {
        let prefix = match self {
            BitcoinNetwork::Mainnet => "bc",
            BitcoinNetwork::Testnet => "tb",
            BitcoinNetwork::Regtest => "bcrt",
        };
        prefix.to_string()
    }
}
