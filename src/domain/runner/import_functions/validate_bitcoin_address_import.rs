use crate::domain::runner::{BitcoinNetwork, CustomEnv};
use bitcoin::{Address, Network, TestnetVersion};
use std::str::FromStr;
use wasmer::{FunctionEnvMut, RuntimeError};

const STATIC_GAS_COST: u64 = 1_000_000;
const GAS_COST_PER_BYTE: u64 = 1_000;

#[derive(Default)]
pub struct ValidateBitcoinAddressImport;

// TODO: Add support for other blockchains
impl ValidateBitcoinAddressImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        address_ptr: u32,
        address_length: u32,
    ) -> Result<u32, RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, STATIC_GAS_COST);

        let address_bytes = instance
            .read_memory(&store, address_ptr as u64, address_length as u64)
            .map_err(|_e| RuntimeError::new("Error reading storage key from memory"))?;

        instance.use_gas(&mut store, address_length as u64 * GAS_COST_PER_BYTE);

        let address_str = String::from_utf8(address_bytes)
            .map_err(|e| RuntimeError::new(format!("Error converting to string: {}", e)))?;

        let result = Self::validate_bitcoin_address(&address_str, &env.network)
            .map_err(|e| RuntimeError::new(e))?;

        Ok(result as u32)
    }

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
    pub fn validate_bitcoin_address(
        address: &str,
        opnet_network: &BitcoinNetwork,
    ) -> Result<bool, String> {
        let network: Network = match opnet_network {
            BitcoinNetwork::Mainnet => Network::Bitcoin,
            BitcoinNetwork::Testnet => Network::Testnet(TestnetVersion::V4),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mainnet_address() {
        let address = "bc1qnghhhgvz5cn8n6x2fy06yzvkuermcm5ljn06gw";

        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            address,
            &BitcoinNetwork::Mainnet,
        );

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_valid_testnet_address() {
        let address = "mym4vP87LdQp9YzRbggpS46fYiQFfR52Nq";

        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            address,
            &BitcoinNetwork::Testnet,
        );

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_valid_p2sh_mainnet_address() {
        let address = "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy";

        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            address,
            &BitcoinNetwork::Mainnet,
        );

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_invalid_address_format() {
        let address = "invalid_address";

        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            address,
            &BitcoinNetwork::Mainnet,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_address_valid_but_wrong_network() {
        let address = "mym4vP87LdQp9YzRbggpS46fYiQFfR52Nq";

        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            address,
            &BitcoinNetwork::Mainnet,
        );

        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_valid_regtest_address() {
        let address = "bcrt1pe0slk2klsxckhf90hvu8g0688rxt9qts6thuxk3u4ymxeejw53gs0xjlhn";

        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            address,
            &BitcoinNetwork::Regtest,
        );

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_valid_regtest_address_segwit() {
        let address = "bcrt1qfqsr3m7vjxheghcvw4ks0fryqxfq8qzjf8fxes";

        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            address,
            &BitcoinNetwork::Regtest,
        );

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_valid_regtest_address_legacy() {
        let address = "mn6KYibk94NhScakhgVPQdGE1bnscugRDG";

        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            address,
            &BitcoinNetwork::Regtest,
        );

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_valid_regtest_address_p2sh() {
        let address = "2MyLLEUGJSusHvPDNHTwYnG9FAJcrQ3VPZY";

        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            address,
            &BitcoinNetwork::Regtest,
        );

        assert_eq!(result, Ok(true));
    }
}
