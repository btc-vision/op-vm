use crate::domain::runner::BitcoinNetwork;
use bitcoin::network::Network;
use bitcoin::Address;
use std::str::FromStr;

/// Validates a Bitcoin address against a specified network.
///
/// # Arguments
///
/// * `address` - A string slice that holds the Bitcoin address.
/// * `network` - The Bitcoin network to validate against (e.g., mainnet, testnet).
///
/// # Returns
///
/// * `Ok(true)` if the address is valid for the specified network.
/// * `Ok(false)` if the address is valid but for a different network.
/// * `Err` if the address is invalid.
pub fn validate_bitcoin_address(address: &str, opnet_network: &BitcoinNetwork) -> Result<bool, String> {
    let network: Network = match opnet_network {
        BitcoinNetwork::Mainnet => Network::Bitcoin,
        BitcoinNetwork::Testnet => Network::Testnet,
        BitcoinNetwork::Regtest => Network::Regtest,
    };

    match Address::from_str(address) {
        Ok(addr) => {
            if addr.is_valid_for_network(network) {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => Err(format!("Invalid address: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mainnet_address() {
        let address = "bc1qnghhhgvz5cn8n6x2fy06yzvkuermcm5ljn06gw";
        assert_eq!(validate_bitcoin_address(address, &BitcoinNetwork::Mainnet), Ok(true));
    }

    #[test]
    fn test_valid_testnet_address() {
        let address = "mym4vP87LdQp9YzRbggpS46fYiQFfR52Nq";
        assert_eq!(validate_bitcoin_address(address, &BitcoinNetwork::Testnet), Ok(true));
    }

    #[test]
    fn test_valid_p2sh_mainnet_address() {
        let address = "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy";
        assert_eq!(validate_bitcoin_address(address, &BitcoinNetwork::Mainnet), Ok(true));
    }

    #[test]
    fn test_invalid_address_format() {
        let address = "invalid_address";
        assert!(validate_bitcoin_address(address, &BitcoinNetwork::Mainnet).is_err());
    }

    #[test]
    fn test_address_valid_but_wrong_network() {
        let address = "mym4vP87LdQp9YzRbggpS46fYiQFfR52Nq";
        assert_eq!(validate_bitcoin_address(address, &BitcoinNetwork::Mainnet), Ok(false));
    }

    #[test]
    fn test_valid_regtest_address() {
        let address = "bcrt1pe0slk2klsxckhf90hvu8g0688rxt9qts6thuxk3u4ymxeejw53gs0xjlhn";
        assert_eq!(validate_bitcoin_address(address, &BitcoinNetwork::Regtest), Ok(true));
    }

    #[test]
    fn test_valid_regtest_address_segwit() {
        let address = "bcrt1qfqsr3m7vjxheghcvw4ks0fryqxfq8qzjf8fxes";
        assert_eq!(validate_bitcoin_address(address, &BitcoinNetwork::Regtest), Ok(true));
    }

    #[test]
    fn test_valid_regtest_address_legacy() {
        let address = "mn6KYibk94NhScakhgVPQdGE1bnscugRDG";
        assert_eq!(validate_bitcoin_address(address, &BitcoinNetwork::Regtest), Ok(true));
    }

    #[test]
    fn test_valid_regtest_address_p2sh() {
        let address = "2MyLLEUGJSusHvPDNHTwYnG9FAJcrQ3VPZY";
        assert_eq!(validate_bitcoin_address(address, &BitcoinNetwork::Regtest), Ok(true));
    }
}
