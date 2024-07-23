pub enum BitcoinNetwork {
    Mainnet,
    Testnet,
    Regtest,
}

impl BitcoinNetwork {
    pub fn address_prefix(&self) -> String {
        let prefix = match self {
            BitcoinNetwork::Mainnet => "bc1",
            BitcoinNetwork::Testnet => "tb1",
            BitcoinNetwork::Regtest => "bcrt1",
        };
        prefix.to_string()
    }
}
