pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

impl Network {
    pub fn address_prefix(&self) -> String {
        let prefix = match self {
            Network::Mainnet => "bc1",
            Network::Testnet => "tb1",
            Network::Regtest => "bcrt1",
        };
        prefix.to_string()
    }
}
