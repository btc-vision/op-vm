use crate::domain::runner::{BitcoinNetwork, CustomEnv};
use bitcoin::{bech32, Address, Network, TestnetVersion};
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
        match opnet_network {
            BitcoinNetwork::OPNetTestnet => Self::validate_opnet_testnet_address(address),
            _ => {
                let network: Network = match opnet_network {
                    BitcoinNetwork::Mainnet => Network::Bitcoin,
                    BitcoinNetwork::Testnet => Network::Testnet(TestnetVersion::V4),
                    BitcoinNetwork::Regtest => Network::Regtest,
                    BitcoinNetwork::OPNetTestnet => unreachable!(),
                };

                match Address::from_str(address) {
                    Ok(addr) => Ok(addr.is_valid_for_network(network)),
                    Err(e) => Err(format!("Invalid address: {}", e)),
                }
            }
        }
    }

    fn validate_opnet_testnet_address(address: &str) -> Result<bool, String> {
        let lower = address.to_lowercase();

        if lower.starts_with("opt1") {
            if address.len() > 90 {
                return Err(format!(
                    "Address too long: {} characters (max 90)",
                    address.len()
                ));
            }

            let (hrp, version, program) = bech32::segwit::decode(address)
                .map_err(|e| format!("Invalid OP_NET bech32 address: {}", e))?;

            if hrp != bech32::Hrp::parse_unchecked("opt") {
                return Ok(false);
            }

            let witness_version = version.to_u8();
            let program_len = program.len();

            if witness_version > 16 {
                return Err(format!(
                    "Invalid witness version: {} (must be 0-16)",
                    witness_version
                ));
            }

            if program_len < 2 || program_len > 40 {
                return Err(format!(
                    "Invalid witness program length: {} (must be 2-40)",
                    program_len
                ));
            }

            if witness_version == 0 && program_len != 20 && program_len != 32 {
                return Err(format!(
                    "Invalid segwit v0 program length: {} (must be 20 or 32)",
                    program_len
                ));
            }

            if witness_version == 1 && program_len != 32 {
                return Err(format!(
                    "Invalid taproot program length: {} (must be 32)",
                    program_len
                ));
            }

            return Ok(true);
        }

        if ["m", "n", "2"].iter().any(|&p| address.starts_with(p)) {
            match Address::from_str(address) {
                Ok(addr) => Ok(addr.is_valid_for_network(Network::Signet)),
                Err(_) => Ok(false),
            }
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Helper: generate valid opt1 bech32/bech32m addresses from raw programs
    // =========================================================================

    fn encode_opt_segwit(version: bech32::Fe32, program: &[u8]) -> String {
        let hrp = bech32::Hrp::parse_unchecked("opt");
        bech32::segwit::encode(hrp, version, program)
            .expect("encoding should succeed for valid inputs")
    }

    // =========================================================================
    // OP_NET testnet: valid p2wpkh (v0, 20-byte program, bech32)
    // =========================================================================

    #[test]
    fn opnet_valid_p2wpkh() {
        let program = [0xab; 20];
        let addr = encode_opt_segwit(bech32::segwit::VERSION_0, &program);
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // OP_NET testnet: valid p2wsh (v0, 32-byte program, bech32)
    // =========================================================================

    #[test]
    fn opnet_valid_p2wsh() {
        let program = [0xcd; 32];
        let addr = encode_opt_segwit(bech32::segwit::VERSION_0, &program);
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // OP_NET testnet: valid p2tr (v1, 32-byte program, bech32m)
    // =========================================================================

    #[test]
    fn opnet_valid_p2tr() {
        let program = [0xef; 32];
        let addr = encode_opt_segwit(bech32::segwit::VERSION_1, &program);
        assert!(addr.to_lowercase().starts_with("opt1p"));
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // OP_NET testnet: valid future witness versions (v2..v16, 32-byte, bech32m)
    // =========================================================================

    #[test]
    fn opnet_valid_future_witness_v2() {
        let program = [0x11; 32];
        let version = bech32::Fe32::try_from(2u8).unwrap();
        let addr = encode_opt_segwit(version, &program);
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn opnet_valid_future_witness_v16() {
        let program = [0x22; 32];
        let version = bech32::Fe32::try_from(16u8).unwrap();
        let addr = encode_opt_segwit(version, &program);
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // OP_NET testnet: future witness version with minimum program length (2 bytes)
    // =========================================================================

    #[test]
    fn opnet_valid_future_witness_min_program() {
        let program = [0xff; 2];
        let version = bech32::Fe32::try_from(2u8).unwrap();
        let addr = encode_opt_segwit(version, &program);
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // OP_NET testnet: future witness version with maximum program length (40 bytes)
    // =========================================================================

    #[test]
    fn opnet_valid_future_witness_max_program() {
        let program = [0xaa; 40];
        let version = bech32::Fe32::try_from(2u8).unwrap();
        let addr = encode_opt_segwit(version, &program);
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // OP_NET testnet: reject v0 with invalid program length (not 20 or 32)
    // =========================================================================

    #[test]
    fn opnet_reject_v0_invalid_program_length() {
        // bech32::segwit::encode itself rejects v0 with length != 20 and != 32,
        // so we cannot produce a valid encoding. Verify decode rejects raw garbage.
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "opt1qqqqqqqqqqqqqqqqqqqqqqqqqqqqkp8gej",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert!(result.is_err());
    }

    // =========================================================================
    // OP_NET testnet: reject v1 (taproot) with program length != 32
    // =========================================================================

    #[test]
    fn opnet_reject_v1_short_program() {
        // segwit::encode rejects v1 with len != 32 at the encode level,
        // so craft a raw bech32m string that bypasses the segwit encoder.
        // A v1 address with a 20-byte program is invalid per BIP-341.
        // segwit::decode itself should reject this, so we expect an error.
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "opt1pqqqqqqqqqqqqqqqqqqqqqqqqqqqgkfn0j",
            &BitcoinNetwork::OPNetTestnet,
        );
        // Either our defensive check or segwit::decode catches it
        assert!(result.is_err() || result == Ok(false));
    }

    // =========================================================================
    // OP_NET testnet: reject wrong HRP (tb1 address on OPNetTestnet)
    // =========================================================================

    #[test]
    fn opnet_reject_tb1_address() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(false));
    }

    // =========================================================================
    // OP_NET testnet: reject mainnet bc1 address
    // =========================================================================

    #[test]
    fn opnet_reject_bc1_address() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(false));
    }

    // =========================================================================
    // OP_NET testnet: reject regtest bcrt1 address
    // =========================================================================

    #[test]
    fn opnet_reject_bcrt1_address() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "bcrt1q2nfxmhd4n3c8834pj72xagvyr9gl57n5r94fsl",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(false));
    }

    // =========================================================================
    // OP_NET testnet: reject mainnet p2pkh (starts with "1")
    // =========================================================================

    #[test]
    fn opnet_reject_mainnet_p2pkh() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(false));
    }

    // =========================================================================
    // OP_NET testnet: reject mainnet p2sh (starts with "3")
    // =========================================================================

    #[test]
    fn opnet_reject_mainnet_p2sh() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(false));
    }

    // =========================================================================
    // OP_NET testnet: accept testnet legacy p2pkh (starts with "m" or "n")
    // =========================================================================

    #[test]
    fn opnet_accept_legacy_p2pkh_m() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "mym4vP87LdQp9YzRbggpS46fYiQFfR52Nq",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn opnet_accept_legacy_p2pkh_n() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "n3GNqMveyvaPvUbH469vDRadqpJMPc84JA",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // OP_NET testnet: accept testnet legacy p2sh (starts with "2")
    // =========================================================================

    #[test]
    fn opnet_accept_legacy_p2sh() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "2MyLLEUGJSusHvPDNHTwYnG9FAJcrQ3VPZY",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // OP_NET testnet: reject empty string
    // =========================================================================

    #[test]
    fn opnet_reject_empty_string() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(false));
    }

    // =========================================================================
    // OP_NET testnet: reject garbage input
    // =========================================================================

    #[test]
    fn opnet_reject_garbage() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "not_an_address_at_all",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(false));
    }

    // =========================================================================
    // OP_NET testnet: reject address exceeding 90 characters
    // =========================================================================

    #[test]
    fn opnet_reject_too_long() {
        let long_addr = format!("opt1q{}", "q".repeat(85));
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &long_addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert!(result.is_err());
    }

    // =========================================================================
    // OP_NET testnet: reject truncated opt1 address (invalid checksum)
    // =========================================================================

    #[test]
    fn opnet_reject_truncated() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "opt1q",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert!(result.is_err());
    }

    // =========================================================================
    // OP_NET testnet: reject opt1 address with invalid bech32 characters
    // =========================================================================

    #[test]
    fn opnet_reject_invalid_bech32_chars() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "opt1qb10",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert!(result.is_err());
    }

    // =========================================================================
    // OP_NET testnet: reject invalid base58 with valid-looking prefix
    // =========================================================================

    #[test]
    fn opnet_reject_invalid_base58_m_prefix() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "m00000000000000000000invalid",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert!(result.is_err());
    }

    // =========================================================================
    // OP_NET testnet: case insensitivity for opt1 prefix
    // =========================================================================

    #[test]
    fn opnet_case_insensitive_uppercase() {
        let program = [0xab; 20];
        let addr = encode_opt_segwit(bech32::segwit::VERSION_0, &program).to_uppercase();
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // OP_NET testnet: reject mixed case (BIP-173 requirement)
    // =========================================================================

    #[test]
    fn opnet_reject_mixed_case() {
        let program = [0xab; 20];
        let addr = encode_opt_segwit(bech32::segwit::VERSION_0, &program);
        // Flip one character to uppercase in the data part to create mixed case
        let mut mixed = addr.clone();
        if let Some(pos) = mixed.find('q') {
            let bytes = unsafe { mixed.as_bytes_mut() };
            if pos + 5 < bytes.len() {
                bytes[pos + 5] = bytes[pos + 5].to_ascii_uppercase();
            }
        }
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &mixed,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert!(result.is_err());
    }

    // =========================================================================
    // OP_NET testnet: reject opt1 with corrupted checksum
    // =========================================================================

    #[test]
    fn opnet_reject_corrupted_checksum() {
        let program = [0xab; 20];
        let mut addr = encode_opt_segwit(bech32::segwit::VERSION_0, &program);
        let len = addr.len();
        let bytes = unsafe { addr.as_bytes_mut() };
        // Flip the last character to corrupt the checksum
        bytes[len - 1] = if bytes[len - 1] == b'q' { b'p' } else { b'q' };
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert!(result.is_err());
    }

    // =========================================================================
    // Cross-network: mainnet address validates on mainnet
    // =========================================================================

    #[test]
    fn mainnet_valid_p2wpkh() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
            &BitcoinNetwork::Mainnet,
        );
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn mainnet_valid_p2pkh() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2",
            &BitcoinNetwork::Mainnet,
        );
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn mainnet_valid_p2sh() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy",
            &BitcoinNetwork::Mainnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // Cross-network: mainnet rejects testnet address
    // =========================================================================

    #[test]
    fn mainnet_reject_testnet_address() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "mym4vP87LdQp9YzRbggpS46fYiQFfR52Nq",
            &BitcoinNetwork::Mainnet,
        );
        assert_eq!(result, Ok(false));
    }

    // =========================================================================
    // Cross-network: mainnet rejects opt1 address
    // =========================================================================

    #[test]
    fn mainnet_reject_opt1_address() {
        let program = [0xab; 20];
        let addr = encode_opt_segwit(bech32::segwit::VERSION_0, &program);
        let result =
            ValidateBitcoinAddressImport::validate_bitcoin_address(&addr, &BitcoinNetwork::Mainnet);
        // Mainnet path uses Address::from_str which does not recognize "opt1"
        assert!(result.is_err());
    }

    // =========================================================================
    // Cross-network: regtest valid address
    // =========================================================================

    #[test]
    fn regtest_valid_p2wpkh() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "bcrt1q2nfxmhd4n3c8834pj72xagvyr9gl57n5r94fsl",
            &BitcoinNetwork::Regtest,
        );
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn regtest_valid_p2tr() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "bcrt1pe0slk2klsxckhf90hvu8g0688rxt9qts6thuxk3u4ymxeejw53gs0xjlhn",
            &BitcoinNetwork::Regtest,
        );
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn regtest_valid_legacy_p2pkh() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "mn6KYibk94NhScakhgVPQdGE1bnscugRDG",
            &BitcoinNetwork::Regtest,
        );
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn regtest_valid_legacy_p2sh() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "2MyLLEUGJSusHvPDNHTwYnG9FAJcrQ3VPZY",
            &BitcoinNetwork::Regtest,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // Cross-network: regtest rejects mainnet
    // =========================================================================

    #[test]
    fn regtest_reject_mainnet_p2wpkh() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
            &BitcoinNetwork::Regtest,
        );
        assert_eq!(result, Ok(false));
    }

    // =========================================================================
    // Testnet valid addresses
    // =========================================================================

    #[test]
    fn testnet_valid_p2wpkh() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx",
            &BitcoinNetwork::Testnet,
        );
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn testnet_valid_legacy_p2pkh() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "mym4vP87LdQp9YzRbggpS46fYiQFfR52Nq",
            &BitcoinNetwork::Testnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // Cross-network: testnet rejects mainnet
    // =========================================================================

    #[test]
    fn testnet_reject_mainnet() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
            &BitcoinNetwork::Testnet,
        );
        assert_eq!(result, Ok(false));
    }

    // =========================================================================
    // Edge: all zeros program (valid encoding, spendable is not our concern)
    // =========================================================================

    #[test]
    fn opnet_valid_all_zeros_p2wpkh() {
        let program = [0x00; 20];
        let addr = encode_opt_segwit(bech32::segwit::VERSION_0, &program);
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn opnet_valid_all_zeros_p2tr() {
        let program = [0x00; 32];
        let addr = encode_opt_segwit(bech32::segwit::VERSION_1, &program);
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // Edge: all 0xff program
    // =========================================================================

    #[test]
    fn opnet_valid_all_ff_p2wpkh() {
        let program = [0xff; 20];
        let addr = encode_opt_segwit(bech32::segwit::VERSION_0, &program);
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn opnet_valid_all_ff_p2tr() {
        let program = [0xff; 32];
        let addr = encode_opt_segwit(bech32::segwit::VERSION_1, &program);
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // Edge: opt1 prefix alone (no data)
    // =========================================================================

    #[test]
    fn opnet_reject_bare_opt1() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "opt1",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert!(result.is_err());
    }

    // =========================================================================
    // Edge: unicode and non-ascii garbage with opt1 prefix
    // =========================================================================

    #[test]
    fn opnet_reject_unicode_garbage() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "opt1q\u{00e9}\u{00e9}\u{00e9}",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert!(result.is_err());
    }

    // =========================================================================
    // Edge: exactly 90 characters (at the limit, should still decode or fail
    // gracefully depending on checksum validity)
    // =========================================================================

    #[test]
    fn opnet_at_90_char_limit() {
        // 90 chars is the boundary. A valid 40-byte v2 program produces ~76 chars
        // with "opt" HRP, well under 90. Verify it works near the limit.
        let program = [0xab; 40];
        let version = bech32::Fe32::try_from(2u8).unwrap();
        let addr = encode_opt_segwit(version, &program);
        assert!(addr.len() <= 90);
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(true));
    }

    // =========================================================================
    // Edge: whitespace padding should be rejected
    // =========================================================================

    #[test]
    fn opnet_reject_leading_whitespace() {
        let program = [0xab; 20];
        let addr = format!(
            " {}",
            encode_opt_segwit(bech32::segwit::VERSION_0, &program)
        );
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn opnet_reject_trailing_whitespace() {
        let program = [0xab; 20];
        let addr = format!(
            "{} ",
            encode_opt_segwit(bech32::segwit::VERSION_0, &program)
        );
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            &addr,
            &BitcoinNetwork::OPNetTestnet,
        );
        assert!(result.is_err());
    }

    // =========================================================================
    // Edge: address that looks like opt1 but has wrong separator position
    // =========================================================================

    #[test]
    fn opnet_reject_fake_opt1_prefix() {
        let result = ValidateBitcoinAddressImport::validate_bitcoin_address(
            "opt10qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq",
            &BitcoinNetwork::OPNetTestnet,
        );
        assert!(result.is_err());
    }

    // =========================================================================
    // Verify all network variants reject invalid_address string
    // =========================================================================

    #[test]
    fn all_networks_reject_invalid_address() {
        let networks = [
            BitcoinNetwork::Mainnet,
            BitcoinNetwork::Testnet,
            BitcoinNetwork::Regtest,
            BitcoinNetwork::OPNetTestnet,
        ];
        for network in &networks {
            let result =
                ValidateBitcoinAddressImport::validate_bitcoin_address("invalid_address", network);
            match network {
                BitcoinNetwork::OPNetTestnet => assert_eq!(result, Ok(false)),
                _ => assert!(result.is_err()),
            }
        }
    }

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
    fn test_valid_opnet_testnet_address() {
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
