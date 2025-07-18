#[derive(Debug)]
pub enum BitcoinNetwork {
    Mainnet,
    Testnet,
    Regtest,
}

// TODO: Add chain_id so it can switch between chain too.
impl BitcoinNetwork {
    #[allow(dead_code)]
    pub fn contract_address_prefix(&self) -> String {
        let prefix = match self {
            BitcoinNetwork::Mainnet => "bc",
            BitcoinNetwork::Testnet => "tb",
            BitcoinNetwork::Regtest => "bcrt",
        };
        prefix.to_string()
    }

    #[allow(dead_code)]
    pub fn is_test_network(&self) -> bool {
        match self {
            BitcoinNetwork::Mainnet => false,
            BitcoinNetwork::Testnet => true,
            BitcoinNetwork::Regtest => true,
        }
    }
}
