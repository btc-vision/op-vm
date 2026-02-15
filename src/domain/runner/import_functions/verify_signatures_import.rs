use crate::domain::runner::{
    verify_mldsa_internal, ConsensusFlags, CustomEnv, InstanceWrapper, MLDSAPublicKeyMetadata,
};
use k256::ecdsa::{
    signature::hazmat::PrehashVerifier, RecoveryId, Signature as EcdsaSignature, VerifyingKey,
};
use secp256k1::schnorr::verify;
use secp256k1::{schnorr, XOnlyPublicKey};
use wasmer::{FunctionEnvMut, RuntimeError, StoreMut};

/// Base host-call overhead charged on every verifySignature invocation
/// regardless of algorithm. Covers memory reads, header parsing, and
/// dispatch logic before any cryptographic work begins.
const STATIC_GAS_COST: u64 = 1_000_000;

/// Schnorr BIP-340 verification: single multi-scalar multiplication
/// (compute sG - eP) plus x-coordinate comparison. Cheapest EC path.
const SCHNORR_VERIFY_GAS_COST: u64 = 40_000_000;

/// ECDSA Bitcoin direct verify: two independent scalar multiplications,
/// point addition, and field comparison. ~1.3-1.4x Schnorr cost.
const ECDSA_BITCOIN_VERIFY_GAS_COST: u64 = 55_000_000;

/// ECDSA Ethereum ecrecover: modular inverse of r, two scalar
/// multiplications, point addition, and full key recovery. Heaviest
/// EC path at ~1.8-2x Schnorr cost.
const ECDSA_ETHEREUM_VERIFY_GAS_COST: u64 = 75_000_000;

/// ML-DSA-44 (security level 2): lattice-based verification over
/// dimension 4x4 matrices with 256-coefficient polynomials.
const MLDSA_44_VERIFY_GAS_COST: u64 = 200_000_000;

/// ML-DSA-65 (security level 3): larger lattice dimension (6x5),
/// more polynomial operations. ~1.75x ML-DSA-44 cost.
const MLDSA_65_VERIFY_GAS_COST: u64 = 350_000_000;

/// ML-DSA-87 (security level 5): largest lattice dimension (8x7),
/// most polynomial operations. ~2.5x ML-DSA-44 cost.
const MLDSA_87_VERIFY_GAS_COST: u64 = 500_000_000;

/// ECDSA sub-type discriminant stored at byte 1 when byte 0 == 0x00.
/// Ethereum: ecrecover model, 65-byte signature (r32 || s32 || v1).
/// Bitcoin:  direct verify model, 64-byte compact signature (r32 || s32).
const ECDSA_SUBTYPE_ETHEREUM: u8 = 0;
const ECDSA_SUBTYPE_BITCOIN: u8 = 1;

/// ECDSA key format discriminant stored at byte 2 when byte 0 == 0x00.
/// Must match ECDSAKeyFormat enum in AssemblyScript guest.
const ECDSA_KEY_FORMAT_COMPRESSED: u8 = 0x00;
const ECDSA_KEY_FORMAT_UNCOMPRESSED: u8 = 0x01;
const ECDSA_KEY_FORMAT_HYBRID: u8 = 0x02;
const ECDSA_KEY_FORMAT_RAW: u8 = 0x03;

/// ML-DSA security level discriminants stored at byte 1 when byte 0 == 0x02.
/// Must match MLDSASecurityLevel enum in AssemblyScript guest.
const MLDSA_LEVEL_44: u8 = 0;
const MLDSA_LEVEL_65: u8 = 1;
const MLDSA_LEVEL_87: u8 = 2;

/// Wire layout from the AS guest:
///
///   Schnorr: [0x01, ...32-byte xonly pubkey]                    -> 33 bytes total
///   MLDSA:   [0x02, level, ...pubkey]                           -> 2 + pubkey_len bytes
///   ECDSA:   [0x00, subtype, format, ...sec1 pubkey material]   -> 3 + pubkey_len bytes
///
/// Signature and message hash are passed as separate buffer pointers.

#[derive(Default)]
pub struct VerifySignatureImport;

impl VerifySignatureImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        public_key_ptr: u32,
        signature_ptr: u32,
        message_ptr: u32,
    ) -> Result<u32, RuntimeError> {
        let (env, store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        Self::execute_inner(
            env.environment_variables.as_ref(),
            store,
            instance,
            public_key_ptr,
            signature_ptr,
            message_ptr,
        )
    }

    /// Core dispatch logic, separated from FunctionEnvMut for testability.
    fn execute_inner(
        env_variables: Option<&crate::domain::runner::EnvironmentVariables>,
        mut store: StoreMut,
        instance: InstanceWrapper,
        public_key_ptr: u32,
        signature_ptr: u32,
        message_ptr: u32,
    ) -> Result<u32, RuntimeError> {
        // Base overhead: memory reads, header parsing, dispatch.
        instance.use_gas(&mut store, STATIC_GAS_COST);

        // All key types use at least a 2-byte header. Schnorr only needs byte 0
        // but reading 2 bytes is harmless since byte 1 is just the first xonly byte.
        let header = instance
            .read_memory(&store, public_key_ptr as u64, 2)
            .map_err(|_e| RuntimeError::new("Error reading public key type header from memory"))?;

        let key_type = header
            .get(0)
            .copied()
            .ok_or_else(|| RuntimeError::new("Error reading public key type byte"))?;

        let env_variables = env_variables
            .ok_or_else(|| RuntimeError::new("Environment variables not found"))?;

        let unsafe_quantum_signatures_allowed =
            env_variables.is_consensus_flag_set(ConsensusFlags::UNSAFE_QUANTUM_SIGNATURES_ALLOWED);

        match key_type {
            // 0x00 = ECDSA (Ethereum or Bitcoin sub-type at byte 1)
            0 => {
                if !unsafe_quantum_signatures_allowed {
                    return Err(RuntimeError::new(
                        "ECDSA signature verification is not allowed by consensus rules",
                    ));
                }

                let ecdsa_subtype = header
                    .get(1)
                    .copied()
                    .ok_or_else(|| RuntimeError::new("Error reading ECDSA sub-type from memory"))?;

                match ecdsa_subtype {
                    ECDSA_SUBTYPE_ETHEREUM => {
                        instance.use_gas(&mut store, ECDSA_ETHEREUM_VERIFY_GAS_COST);
                        Self::verify_ecdsa_ethereum(
                            store,
                            instance,
                            public_key_ptr,
                            signature_ptr,
                            message_ptr,
                        )
                    }
                    ECDSA_SUBTYPE_BITCOIN => {
                        instance.use_gas(&mut store, ECDSA_BITCOIN_VERIFY_GAS_COST);
                        Self::verify_ecdsa_bitcoin(
                            store,
                            instance,
                            public_key_ptr,
                            signature_ptr,
                            message_ptr,
                        )
                    }
                    _ => Err(RuntimeError::new("Unsupported ECDSA sub-type")),
                }
            }
            // 0x01 = Schnorr
            1 => {
                if !unsafe_quantum_signatures_allowed {
                    return Err(RuntimeError::new(
                        "Schnorr signature verification is not allowed by consensus rules",
                    ));
                }

                instance.use_gas(&mut store, SCHNORR_VERIFY_GAS_COST);

                Self::verify_schnorr_signature(
                    store,
                    instance,
                    public_key_ptr,
                    signature_ptr,
                    message_ptr,
                )
            }
            // 0x02 = ML-DSA
            2 => {
                let mldsa_level = header.get(1).copied().ok_or_else(|| {
                    RuntimeError::new("Error reading MLDSA public key level from memory")
                })?;

                let mldsa_gas = match mldsa_level {
                    MLDSA_LEVEL_44 => MLDSA_44_VERIFY_GAS_COST,
                    MLDSA_LEVEL_65 => MLDSA_65_VERIFY_GAS_COST,
                    MLDSA_LEVEL_87 => MLDSA_87_VERIFY_GAS_COST,
                    _ => {
                        return Err(RuntimeError::new(
                            "Error determining MLDSA gas cost from level",
                        ));
                    }
                };

                instance.use_gas(&mut store, mldsa_gas);

                Self::verify_mldsa_signature(
                    store,
                    instance,
                    public_key_ptr,
                    signature_ptr,
                    message_ptr,
                    mldsa_level,
                )
            }
            _ => Err(RuntimeError::new("Unsupported public key type")),
        }
    }

    /// Reads the ECDSA public key material from memory using the explicit
    /// format discriminant at byte 2 of the wire buffer.
    ///
    /// Wire layout for ECDSA: [type(1), subtype(1), format(1), ...key_material]
    ///
    /// Format discriminants (must match ECDSAKeyFormat in AS guest):
    ///   0x00 -> Compressed:   33-byte SEC1 (0x02/0x03 prefix)
    ///   0x01 -> Uncompressed: 65-byte SEC1 (0x04 prefix)
    ///   0x02 -> Hybrid:       65-byte SEC1 (0x06/0x07 prefix, rewritten to 0x04)
    ///   0x03 -> Raw:          64-byte X||Y (no prefix, 0x04 prepended for parsing)
    fn read_ecdsa_public_key(
        store: &StoreMut,
        instance: &InstanceWrapper,
        public_key_ptr: u32,
    ) -> Result<VerifyingKey, RuntimeError> {
        let format_byte = instance
            .read_memory(store, (public_key_ptr as u64) + 2, 1)
            .map_err(|_| RuntimeError::new("Error reading ECDSA key format byte"))?;

        let format = format_byte
            .get(0)
            .copied()
            .ok_or_else(|| RuntimeError::new("Error reading ECDSA key format byte"))?;

        let key_offset = (public_key_ptr as u64) + 3;

        match format {
            ECDSA_KEY_FORMAT_COMPRESSED => {
                let pk_bytes = instance
                    .read_memory(store, key_offset, 33)
                    .map_err(|_| RuntimeError::new("Error reading compressed ECDSA public key"))?;

                VerifyingKey::from_sec1_bytes(&pk_bytes)
                    .map_err(|e| RuntimeError::new(format!("Invalid compressed ECDSA key: {}", e)))
            }
            ECDSA_KEY_FORMAT_UNCOMPRESSED => {
                let pk_bytes = instance.read_memory(store, key_offset, 65).map_err(|_| {
                    RuntimeError::new("Error reading uncompressed ECDSA public key")
                })?;

                VerifyingKey::from_sec1_bytes(&pk_bytes).map_err(|e| {
                    RuntimeError::new(format!("Invalid uncompressed ECDSA key: {}", e))
                })
            }
            ECDSA_KEY_FORMAT_HYBRID => {
                let mut pk_bytes = instance
                    .read_memory(store, key_offset, 65)
                    .map_err(|_| RuntimeError::new("Error reading hybrid ECDSA public key"))?;

                pk_bytes[0] = 0x04;

                VerifyingKey::from_sec1_bytes(&pk_bytes)
                    .map_err(|e| RuntimeError::new(format!("Invalid hybrid ECDSA key: {}", e)))
            }
            ECDSA_KEY_FORMAT_RAW => {
                let raw_bytes = instance
                    .read_memory(store, key_offset, 64)
                    .map_err(|_| RuntimeError::new("Error reading raw ECDSA public key"))?;

                let mut sec1_buf = Vec::with_capacity(65);
                sec1_buf.push(0x04);
                sec1_buf.extend_from_slice(&raw_bytes);

                VerifyingKey::from_sec1_bytes(&sec1_buf)
                    .map_err(|e| RuntimeError::new(format!("Invalid raw ECDSA key: {}", e)))
            }
            _ => Err(RuntimeError::new(format!(
                "Unsupported ECDSA public key format: 0x{:02X}",
                format,
            ))),
        }
    }

    /// Ethereum-style ECDSA verification (ecrecover model).
    ///
    /// Signature wire format: 65 bytes -> r (32) || s (32) || v (1).
    /// v is the recovery identifier: 0/1 raw or 27/28 EIP-155.
    ///
    /// Recovers the signer public key from (hash, r, s, v) and compares
    /// against the provided key material. Low-S normalization is NOT applied
    /// before recovery because flipping S changes which key gets recovered,
    /// breaking valid pre-EIP-2 signatures.
    fn verify_ecdsa_ethereum(
        store: StoreMut,
        instance: InstanceWrapper,
        public_key_ptr: u32,
        signature_ptr: u32,
        message_ptr: u32,
    ) -> Result<u32, RuntimeError> {
        let sig_bytes: [u8; 65] = instance
            .read_memory(&store, signature_ptr as u64, 65)
            .map_err(|_e| RuntimeError::new("Error reading ECDSA signature from memory"))?
            .try_into()
            .map_err(|_e| RuntimeError::new("ECDSA signature must be exactly 65 bytes"))?;

        let message_hash: [u8; 32] = instance
            .read_memory(&store, message_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading ECDSA message hash from memory"))?
            .try_into()
            .map_err(|_e| RuntimeError::new("ECDSA message hash must be exactly 32 bytes"))?;

        let signature = match EcdsaSignature::from_slice(&sig_bytes[..64]) {
            Ok(sig) => sig,
            Err(_) => return Ok(0),
        };

        let v_byte = sig_bytes[64];
        let recovery_id_value = match v_byte {
            0 | 1 => v_byte,
            27 | 28 => v_byte - 27,
            _ => return Ok(0),
        };

        let recovery_id = match RecoveryId::from_byte(recovery_id_value) {
            Some(id) => id,
            None => return Ok(0),
        };

        let recovered_key =
            match VerifyingKey::recover_from_prehash(&message_hash, &signature, recovery_id) {
                Ok(key) => key,
                Err(_) => return Ok(0),
            };

        let provided_key = match Self::read_ecdsa_public_key(&store, &instance, public_key_ptr) {
            Ok(key) => key,
            Err(_) => return Ok(0),
        };

        Ok((recovered_key == provided_key) as u32)
    }

    /// Bitcoin-style ECDSA verification (direct verification model).
    ///
    /// Signature wire format: 64 bytes compact -> r (32) || s (32).
    /// No recovery id needed since we verify directly against the provided key.
    ///
    /// Low-S normalization is enforced per BIP-0062 to prevent signature
    /// malleability attacks that can mutate transaction IDs. normalize_s()
    /// returns the signature with S replaced by (n - S) if S > n/2, or
    /// returns the signature unchanged if S is already in the lower half.
    fn verify_ecdsa_bitcoin(
        store: StoreMut,
        instance: InstanceWrapper,
        public_key_ptr: u32,
        signature_ptr: u32,
        message_ptr: u32,
    ) -> Result<u32, RuntimeError> {
        let sig_bytes: [u8; 64] = instance
            .read_memory(&store, signature_ptr as u64, 64)
            .map_err(|_e| RuntimeError::new("Error reading ECDSA signature from memory"))?
            .try_into()
            .map_err(|_e| RuntimeError::new("ECDSA signature must be exactly 64 bytes"))?;

        let message_hash: [u8; 32] = instance
            .read_memory(&store, message_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading ECDSA message hash from memory"))?
            .try_into()
            .map_err(|_e| RuntimeError::new("ECDSA message hash must be exactly 32 bytes"))?;

        let signature = match EcdsaSignature::from_slice(&sig_bytes) {
            Ok(sig) => sig,
            Err(_) => return Ok(0),
        };

        // BIP-0062: enforce canonical low-S form. normalize_s() returns Self,
        // always valid. If S was already low it returns an identical copy.
        let signature = signature.normalize_s();

        let verifying_key = match Self::read_ecdsa_public_key(&store, &instance, public_key_ptr) {
            Ok(key) => key,
            Err(_) => return Ok(0),
        };

        let result = verifying_key.verify_prehash(&message_hash, &signature);
        Ok(result.is_ok() as u32)
    }

    fn verify_schnorr_signature(
        store: StoreMut,
        instance: InstanceWrapper,
        public_key_ptr: u32,
        signature_ptr: u32,
        message_ptr: u32,
    ) -> Result<u32, RuntimeError> {
        // Schnorr layout: [0x01, ...32-byte xonly pubkey] = 33 bytes
        let public_key_bytes: [u8; 33] = instance
            .read_memory(&store, public_key_ptr as u64, 33)
            .map_err(|_e| RuntimeError::new("Error reading Schnorr public key from memory"))?
            .try_into()
            .map_err(|_e| RuntimeError::new("Error reading Schnorr public key from memory"))?;

        // Skip byte 0 (type tag), remaining 32 bytes are the xonly public key
        let xonly_bytes: [u8; 32] = public_key_bytes[1..33]
            .try_into()
            .map_err(|_e| RuntimeError::new("Error converting Schnorr public key to xonly"))?;

        let signature_bytes: [u8; 64] = instance
            .read_memory(&store, signature_ptr as u64, 64)
            .map_err(|_e| RuntimeError::new("Error reading Schnorr signature from memory"))?
            .try_into()
            .map_err(|_e| RuntimeError::new("Error reading Schnorr signature from memory"))?;

        let message_bytes = instance
            .read_memory(&store, message_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading Schnorr message from memory"))?;

        let xonly_public_key = XOnlyPublicKey::from_byte_array(xonly_bytes)
            .map_err(|e| RuntimeError::new(format!("Invalid Schnorr public key: {}", e)))?;

        let signature = schnorr::Signature::from_byte_array(signature_bytes);
        let result = verify(&signature, &message_bytes, &xonly_public_key);
        Ok(result.is_ok() as u32)
    }

    fn verify_mldsa_signature(
        store: StoreMut,
        instance: InstanceWrapper,
        public_key_ptr: u32,
        signature_ptr: u32,
        message_ptr: u32,
        mldsa_level: u8,
    ) -> Result<u32, RuntimeError> {
        let public_key_metadata =
            MLDSAPublicKeyMetadata::from_level(mldsa_level).ok_or_else(|| {
                RuntimeError::new("Error determining MLDSA public key metadata from level")
            })?;

        // ML-DSA layout: [0x02, level, ...pubkey] -> skip first 2 bytes
        let public_key_bytes = instance
            .read_memory(
                &store,
                public_key_ptr as u64,
                2 + public_key_metadata.as_u64(),
            )
            .map_err(|e| RuntimeError::new(format!("Error reading ML-DSA public key: {:?}", e)))?;

        let public_key_bytes = &public_key_bytes[2..];

        let signature_bytes = instance
            .read_memory(
                &store,
                signature_ptr as u64,
                public_key_metadata.signature_len() as u64,
            )
            .map_err(|e| RuntimeError::new(format!("Error reading ML-DSA signature: {:?}", e)))?;

        let message_bytes = instance
            .read_memory(&store, message_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading ML-DSA message from memory"))?;

        let verification_result = std::panic::catch_unwind(|| {
            verify_mldsa_internal(
                &public_key_metadata,
                &public_key_bytes,
                &signature_bytes,
                &message_bytes,
            )
        });

        match verification_result {
            Ok(result) => Ok(result as u32),
            Err(e) => {
                if cfg!(debug_assertions) {
                    eprintln!("ML-DSA signature verification panicked: {:?}", e);
                }

                Ok(0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ECDSA_KEY_FORMAT_COMPRESSED, ECDSA_KEY_FORMAT_HYBRID, ECDSA_KEY_FORMAT_RAW,
        ECDSA_KEY_FORMAT_UNCOMPRESSED,
    };
    use k256::ecdsa::{
        signature::hazmat::PrehashVerifier, RecoveryId, Signature as EcdsaSignature, SigningKey,
        VerifyingKey,
    };
    use secp256k1::{schnorr, XOnlyPublicKey};

    // =========================================================================
    // Deterministic key generation from fixed seeds. No OsRng, no rand crate.
    // =========================================================================

    fn signing_key_from_seed(seed: &[u8; 32]) -> SigningKey {
        SigningKey::from_bytes(seed.into()).expect("seed must be a valid secp256k1 scalar")
    }

    fn default_signing_key() -> SigningKey {
        signing_key_from_seed(&[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0x20,
        ])
    }

    fn alternate_signing_key() -> SigningKey {
        signing_key_from_seed(&[
            0xaa, 0xbb, 0xcc, 0xdd, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0xff,
        ])
    }

    fn signing_key_at_index(idx: u8) -> SigningKey {
        let mut seed = [0x10u8; 32];
        seed[0] = idx;
        seed[31] = idx.wrapping_add(0x80);
        signing_key_from_seed(&seed)
    }

    fn test_message_hash() -> [u8; 32] {
        [
            0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x66, 0x77, 0x88, 0x99,
        ]
    }

    // =========================================================================
    // Ethereum signing: 65-byte wire r(32) || s(32) || v(1)
    // =========================================================================

    fn sign_ethereum(sk: &SigningKey, hash: &[u8; 32]) -> ([u8; 65], RecoveryId) {
        let (sig, rid) = sk
            .sign_prehash_recoverable(hash)
            .expect("signing must succeed");

        let mut wire = [0u8; 65];
        wire[..64].copy_from_slice(&sig.to_bytes());
        wire[64] = rid.to_byte();
        (wire, rid)
    }

    // =========================================================================
    // Bitcoin signing: 64-byte compact r(32) || s(32), low-S normalized
    // =========================================================================

    fn sign_bitcoin(sk: &SigningKey, hash: &[u8; 32]) -> [u8; 64] {
        let (sig, _) = sk
            .sign_prehash_recoverable(hash)
            .expect("signing must succeed");

        let normalized = sig.normalize_s();
        let mut compact = [0u8; 64];
        compact.copy_from_slice(&normalized.to_bytes());
        compact
    }

    // =========================================================================
    // Key format helpers. Uses to_sec1_point (k256 0.14 renamed from
    // to_encoded_point). Requires k256 "arithmetic" feature in dev-deps.
    // =========================================================================

    fn pubkey_compressed(vk: &VerifyingKey) -> Vec<u8> {
        vk.to_sec1_point(true).as_bytes().to_vec()
    }

    fn pubkey_uncompressed_65(vk: &VerifyingKey) -> Vec<u8> {
        vk.to_sec1_point(false).as_bytes().to_vec()
    }

    fn pubkey_raw_64(vk: &VerifyingKey) -> Vec<u8> {
        let full = vk.to_sec1_point(false);
        full.as_bytes()[1..].to_vec()
    }

    fn pubkey_hybrid(vk: &VerifyingKey) -> Vec<u8> {
        let compressed = vk.to_sec1_point(true);
        let prefix_byte = compressed.as_bytes()[0];
        let mut uncompressed = pubkey_uncompressed_65(vk);
        uncompressed[0] = if prefix_byte == 0x02 { 0x06 } else { 0x07 };
        uncompressed
    }

    // =========================================================================
    // Mirror of Rust host's read_ecdsa_public_key parsing logic.
    // Uses the explicit format discriminant, not byte-sniffing.
    // =========================================================================

    fn parse_public_key_with_format(format: u8, pk_bytes: &[u8]) -> Result<VerifyingKey, String> {
        match format {
            ECDSA_KEY_FORMAT_COMPRESSED => {
                if pk_bytes.len() != 33 {
                    return Err(format!("compressed key len {} != 33", pk_bytes.len()));
                }
                VerifyingKey::from_sec1_bytes(pk_bytes).map_err(|e| format!("{}", e))
            }
            ECDSA_KEY_FORMAT_UNCOMPRESSED => {
                if pk_bytes.len() != 65 {
                    return Err(format!("uncompressed key len {} != 65", pk_bytes.len()));
                }
                VerifyingKey::from_sec1_bytes(pk_bytes).map_err(|e| format!("{}", e))
            }
            ECDSA_KEY_FORMAT_HYBRID => {
                if pk_bytes.len() != 65 {
                    return Err(format!("hybrid key len {} != 65", pk_bytes.len()));
                }
                let mut rewritten = pk_bytes.to_vec();
                rewritten[0] = 0x04;
                VerifyingKey::from_sec1_bytes(&rewritten).map_err(|e| format!("{}", e))
            }
            ECDSA_KEY_FORMAT_RAW => {
                if pk_bytes.len() != 64 {
                    return Err(format!("raw key len {} != 64", pk_bytes.len()));
                }
                let mut buf = Vec::with_capacity(65);
                buf.push(0x04);
                buf.extend_from_slice(pk_bytes);
                VerifyingKey::from_sec1_bytes(&buf).map_err(|e| format!("{}", e))
            }
            _ => Err(format!("unsupported format: 0x{:02X}", format)),
        }
    }

    /// Derives the format tag the same way the AS guest does, based on key
    /// length and prefix byte. This mirrors internalVerifyECDSA's logic.
    fn derive_format_tag(pk_bytes: &[u8]) -> u8 {
        let len = pk_bytes.len();
        if len == 33 {
            ECDSA_KEY_FORMAT_COMPRESSED
        } else if len == 64 {
            ECDSA_KEY_FORMAT_RAW
        } else if pk_bytes[0] == 0x06 || pk_bytes[0] == 0x07 {
            ECDSA_KEY_FORMAT_HYBRID
        } else {
            ECDSA_KEY_FORMAT_UNCOMPRESSED
        }
    }

    /// Convenience wrapper: derive format tag from key bytes, then parse.
    fn parse_public_key(pk_bytes: &[u8]) -> Result<VerifyingKey, String> {
        if pk_bytes.is_empty() {
            return Err("empty key".into());
        }
        let format = derive_format_tag(pk_bytes);
        parse_public_key_with_format(format, pk_bytes)
    }

    // =========================================================================
    // Hex decode helper for BIP-340 test vectors
    // =========================================================================

    fn hex_decode(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    fn hex_decode_32(s: &str) -> [u8; 32] {
        let v = hex_decode(s);
        let mut out = [0u8; 32];
        out.copy_from_slice(&v);
        out
    }

    fn hex_decode_64(s: &str) -> [u8; 64] {
        let v = hex_decode(s);
        let mut out = [0u8; 64];
        out.copy_from_slice(&v);
        out
    }

    // =========================================================================
    // ETHEREUM ECDSA: ecrecover model
    // =========================================================================

    #[test]
    fn eth_ecrecover_compressed_key_matches() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let (sig, rid) = sign_ethereum(&sk, &msg);

        let signature = EcdsaSignature::from_slice(&sig[..64]).unwrap();
        let recovered = VerifyingKey::recover_from_prehash(&msg, &signature, rid).unwrap();

        let provided = parse_public_key(&pubkey_compressed(&vk)).unwrap();
        assert_eq!(recovered, provided);
    }

    #[test]
    fn eth_ecrecover_uncompressed_key_matches() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let (sig, rid) = sign_ethereum(&sk, &msg);

        let signature = EcdsaSignature::from_slice(&sig[..64]).unwrap();
        let recovered = VerifyingKey::recover_from_prehash(&msg, &signature, rid).unwrap();

        let provided = parse_public_key(&pubkey_uncompressed_65(&vk)).unwrap();
        assert_eq!(recovered, provided);
    }

    #[test]
    fn eth_ecrecover_raw_64_key_matches() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let (sig, rid) = sign_ethereum(&sk, &msg);

        let signature = EcdsaSignature::from_slice(&sig[..64]).unwrap();
        let recovered = VerifyingKey::recover_from_prehash(&msg, &signature, rid).unwrap();

        let provided = parse_public_key(&pubkey_raw_64(&vk)).unwrap();
        assert_eq!(recovered, provided);
    }

    #[test]
    fn eth_ecrecover_hybrid_key_matches() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let (sig, rid) = sign_ethereum(&sk, &msg);

        let signature = EcdsaSignature::from_slice(&sig[..64]).unwrap();
        let recovered = VerifyingKey::recover_from_prehash(&msg, &signature, rid).unwrap();

        let hybrid = pubkey_hybrid(&vk);
        assert!(hybrid[0] == 0x06 || hybrid[0] == 0x07);
        let provided = parse_public_key(&hybrid).unwrap();
        assert_eq!(recovered, provided);
    }

    #[test]
    fn eth_v_byte_raw_0_or_1() {
        let sk = default_signing_key();
        let msg = test_message_hash();
        let (sig, rid) = sign_ethereum(&sk, &msg);

        let raw_v = sig[64];
        assert!(raw_v <= 1, "raw recovery id must be 0 or 1");
        assert_eq!(raw_v, rid.to_byte());
    }

    #[test]
    fn eth_v_byte_eip155_27_28_mapping() {
        let sk = default_signing_key();
        let msg = test_message_hash();
        let (_, rid) = sign_ethereum(&sk, &msg);

        let v_eip155 = rid.to_byte() + 27;
        assert!(v_eip155 == 27 || v_eip155 == 28);

        let mapped = v_eip155 - 27;
        let parsed_rid = RecoveryId::from_byte(mapped).unwrap();
        assert_eq!(parsed_rid, rid);

        let (sig_wire, _) = sign_ethereum(&sk, &msg);
        let signature = EcdsaSignature::from_slice(&sig_wire[..64]).unwrap();
        let recovered = VerifyingKey::recover_from_prehash(&msg, &signature, parsed_rid).unwrap();
        assert_eq!(recovered, *sk.verifying_key());
    }

    #[test]
    fn eth_invalid_v_values_rejected() {
        for v in [2u8, 3, 26, 29, 30, 100, 255] {
            let accepted = matches!(v, 0 | 1 | 27 | 28);
            assert!(!accepted, "v={} must be rejected", v);
        }
    }

    #[test]
    fn eth_wrong_message_does_not_match() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let (sig, rid) = sign_ethereum(&sk, &msg);

        let mut bad = msg;
        bad[0] ^= 0xff;

        let signature = EcdsaSignature::from_slice(&sig[..64]).unwrap();
        match VerifyingKey::recover_from_prehash(&bad, &signature, rid) {
            Ok(recovered) => assert_ne!(recovered, vk),
            Err(_) => {}
        }
    }

    #[test]
    fn eth_wrong_key_does_not_match() {
        let sk = default_signing_key();
        let other = alternate_signing_key();
        let msg = test_message_hash();
        let (sig, rid) = sign_ethereum(&sk, &msg);

        let signature = EcdsaSignature::from_slice(&sig[..64]).unwrap();
        let recovered = VerifyingKey::recover_from_prehash(&msg, &signature, rid).unwrap();
        assert_ne!(recovered, *other.verifying_key());
    }

    #[test]
    fn eth_invalid_r_zero() {
        let mut bad = [0u8; 64];
        bad[32] = 0x01;
        assert!(EcdsaSignature::from_slice(&bad).is_err());
    }

    #[test]
    fn eth_invalid_s_zero() {
        let mut bad = [0u8; 64];
        bad[3] = 0x01;
        assert!(EcdsaSignature::from_slice(&bad).is_err());
    }

    #[test]
    fn eth_recovery_id_boundaries() {
        assert!(RecoveryId::from_byte(0).is_some());
        assert!(RecoveryId::from_byte(1).is_some());
        assert!(RecoveryId::from_byte(2).is_some());
        assert!(RecoveryId::from_byte(3).is_some());
        assert!(RecoveryId::from_byte(4).is_none());
        assert!(RecoveryId::from_byte(128).is_none());
        assert!(RecoveryId::from_byte(255).is_none());
    }

    #[test]
    fn eth_recovery_id_0_vs_1_different_keys() {
        let sk = default_signing_key();
        let msg = test_message_hash();
        let (sig_wire, actual_rid) = sign_ethereum(&sk, &msg);

        let signature = EcdsaSignature::from_slice(&sig_wire[..64]).unwrap();

        let other_rid = if actual_rid.to_byte() == 0 {
            RecoveryId::from_byte(1).unwrap()
        } else {
            RecoveryId::from_byte(0).unwrap()
        };

        let correct = VerifyingKey::recover_from_prehash(&msg, &signature, actual_rid).unwrap();
        match VerifyingKey::recover_from_prehash(&msg, &signature, other_rid) {
            Ok(wrong) => assert_ne!(correct, wrong),
            Err(_) => {}
        }
    }

    #[test]
    fn eth_ecrecover_20_deterministic_keys() {
        let msg = test_message_hash();
        for idx in 1u8..=20 {
            let sk = signing_key_at_index(idx);
            let vk = *sk.verifying_key();
            let (sig, rid) = sign_ethereum(&sk, &msg);

            let signature = EcdsaSignature::from_slice(&sig[..64]).unwrap();
            let recovered = VerifyingKey::recover_from_prehash(&msg, &signature, rid).unwrap();
            assert_eq!(recovered, vk, "ecrecover mismatch at index {}", idx);
        }
    }

    // =========================================================================
    // BITCOIN ECDSA: direct verify model
    // =========================================================================

    #[test]
    fn btc_verify_compressed_key() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);

        let signature = EcdsaSignature::from_slice(&sig).unwrap().normalize_s();
        assert!(vk.verify_prehash(&msg, &signature).is_ok());
    }

    #[test]
    fn btc_verify_uncompressed_key() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);

        let parsed = parse_public_key(&pubkey_uncompressed_65(&vk)).unwrap();
        let signature = EcdsaSignature::from_slice(&sig).unwrap().normalize_s();
        assert!(parsed.verify_prehash(&msg, &signature).is_ok());
    }

    #[test]
    fn btc_verify_raw_64_key() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);

        let parsed = parse_public_key(&pubkey_raw_64(&vk)).unwrap();
        let signature = EcdsaSignature::from_slice(&sig).unwrap().normalize_s();
        assert!(parsed.verify_prehash(&msg, &signature).is_ok());
    }

    #[test]
    fn btc_verify_hybrid_key() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);

        let parsed = parse_public_key(&pubkey_hybrid(&vk)).unwrap();
        let signature = EcdsaSignature::from_slice(&sig).unwrap().normalize_s();
        assert!(parsed.verify_prehash(&msg, &signature).is_ok());
    }

    #[test]
    fn btc_wrong_message_fails() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);

        let mut bad = msg;
        bad[0] ^= 0xff;

        let signature = EcdsaSignature::from_slice(&sig).unwrap().normalize_s();
        assert!(vk.verify_prehash(&bad, &signature).is_err());
    }

    #[test]
    fn btc_wrong_key_fails() {
        let sk = default_signing_key();
        let other = alternate_signing_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);

        let signature = EcdsaSignature::from_slice(&sig).unwrap().normalize_s();
        assert!(other
            .verifying_key()
            .verify_prehash(&msg, &signature)
            .is_err());
    }

    #[test]
    fn btc_normalize_s_idempotent() {
        let sk = default_signing_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);

        let parsed = EcdsaSignature::from_slice(&sig).unwrap();
        let once = parsed.normalize_s();
        let twice = once.normalize_s();
        assert_eq!(once.to_bytes(), twice.to_bytes());
    }

    #[test]
    fn btc_high_s_normalized_still_verifies() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();

        let (raw_sig, _) = sk.sign_prehash_recoverable(&msg).unwrap();
        let normalized = raw_sig.normalize_s();
        assert!(vk.verify_prehash(&msg, &normalized).is_ok());
    }

    #[test]
    fn btc_verify_20_deterministic_keys() {
        let msg = test_message_hash();
        for idx in 1u8..=20 {
            let sk = signing_key_at_index(idx);
            let vk = *sk.verifying_key();
            let sig = sign_bitcoin(&sk, &msg);

            let signature = EcdsaSignature::from_slice(&sig).unwrap().normalize_s();
            assert!(
                vk.verify_prehash(&msg, &signature).is_ok(),
                "btc verify failed at index {}",
                idx,
            );
        }
    }

    #[test]
    fn btc_normalize_s_consistent_across_50_keys() {
        let msg = test_message_hash();
        for idx in 1u8..=50 {
            let sk = signing_key_at_index(idx);
            let (sig, _) = sk.sign_prehash_recoverable(&msg).unwrap();
            let normalized = sig.normalize_s();
            let double = normalized.normalize_s();

            assert_eq!(
                normalized.to_bytes(),
                double.to_bytes(),
                "normalize_s not idempotent at index {}",
                idx,
            );

            assert!(
                sk.verifying_key().verify_prehash(&msg, &normalized).is_ok(),
                "normalized sig failed at index {}",
                idx,
            );
        }
    }

    // =========================================================================
    // CROSS-FORMAT KEY CONSISTENCY
    // =========================================================================

    #[test]
    fn all_four_formats_parse_to_equal_key() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();

        let from_c = parse_public_key(&pubkey_compressed(&vk)).unwrap();
        let from_u = parse_public_key(&pubkey_uncompressed_65(&vk)).unwrap();
        let from_r = parse_public_key(&pubkey_raw_64(&vk)).unwrap();
        let from_h = parse_public_key(&pubkey_hybrid(&vk)).unwrap();

        assert_eq!(from_c, vk);
        assert_eq!(from_u, vk);
        assert_eq!(from_r, vk);
        assert_eq!(from_h, vk);
        assert_eq!(from_c, from_u);
        assert_eq!(from_u, from_r);
        assert_eq!(from_r, from_h);
    }

    #[test]
    fn all_formats_verify_same_eth_sig() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let (sig, rid) = sign_ethereum(&sk, &msg);

        let signature = EcdsaSignature::from_slice(&sig[..64]).unwrap();
        let recovered = VerifyingKey::recover_from_prehash(&msg, &signature, rid).unwrap();

        let formats: [Vec<u8>; 4] = [
            pubkey_compressed(&vk),
            pubkey_uncompressed_65(&vk),
            pubkey_raw_64(&vk),
            pubkey_hybrid(&vk),
        ];

        for (i, fmt) in formats.iter().enumerate() {
            let parsed = parse_public_key(fmt).unwrap_or_else(|e| {
                panic!("format {} parse failed: {}", i, e);
            });
            assert_eq!(recovered, parsed, "format {} mismatch after ecrecover", i);
        }
    }

    #[test]
    fn all_formats_verify_same_btc_sig() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);

        let signature = EcdsaSignature::from_slice(&sig).unwrap().normalize_s();

        let formats: [Vec<u8>; 4] = [
            pubkey_compressed(&vk),
            pubkey_uncompressed_65(&vk),
            pubkey_raw_64(&vk),
            pubkey_hybrid(&vk),
        ];

        for (i, fmt) in formats.iter().enumerate() {
            let parsed = parse_public_key(fmt).unwrap_or_else(|e| {
                panic!("format {} parse failed: {}", i, e);
            });
            assert!(
                parsed.verify_prehash(&msg, &signature).is_ok(),
                "format {} btc verify failed",
                i,
            );
        }
    }

    // =========================================================================
    // FORMAT TAG DERIVATION (mirrors AS guest logic)
    // =========================================================================

    #[test]
    fn derive_format_tag_compressed() {
        let vk = *default_signing_key().verifying_key();
        let pk = pubkey_compressed(&vk);
        assert_eq!(derive_format_tag(&pk), ECDSA_KEY_FORMAT_COMPRESSED);
        assert_eq!(pk.len(), 33);
    }

    #[test]
    fn derive_format_tag_uncompressed() {
        let vk = *default_signing_key().verifying_key();
        let pk = pubkey_uncompressed_65(&vk);
        assert_eq!(derive_format_tag(&pk), ECDSA_KEY_FORMAT_UNCOMPRESSED);
        assert_eq!(pk.len(), 65);
        assert_eq!(pk[0], 0x04);
    }

    #[test]
    fn derive_format_tag_hybrid() {
        let vk = *default_signing_key().verifying_key();
        let pk = pubkey_hybrid(&vk);
        assert_eq!(derive_format_tag(&pk), ECDSA_KEY_FORMAT_HYBRID);
        assert_eq!(pk.len(), 65);
        assert!(pk[0] == 0x06 || pk[0] == 0x07);
    }

    #[test]
    fn derive_format_tag_raw() {
        let vk = *default_signing_key().verifying_key();
        let pk = pubkey_raw_64(&vk);
        assert_eq!(derive_format_tag(&pk), ECDSA_KEY_FORMAT_RAW);
        assert_eq!(pk.len(), 64);
    }

    /// Regression test: raw 64-byte key whose X starts with 0x02 must still
    /// get format tag Raw (0x03), not Compressed. This was the original bug.
    #[test]
    fn derive_format_tag_raw_key_starting_with_0x02() {
        let mut fake_raw = vec![0x02; 64];
        fake_raw[1] = 0xAB;
        assert_eq!(derive_format_tag(&fake_raw), ECDSA_KEY_FORMAT_RAW);
    }

    #[test]
    fn derive_format_tag_raw_key_starting_with_0x04() {
        let mut fake_raw = vec![0x04; 64];
        fake_raw[1] = 0xCD;
        assert_eq!(derive_format_tag(&fake_raw), ECDSA_KEY_FORMAT_RAW);
    }

    #[test]
    fn derive_format_tag_raw_key_starting_with_0x06() {
        let mut fake_raw = vec![0x06; 64];
        fake_raw[1] = 0xEF;
        assert_eq!(derive_format_tag(&fake_raw), ECDSA_KEY_FORMAT_RAW);
    }

    // =========================================================================
    // EXPLICIT FORMAT PARSING (simulates Rust host reading format byte)
    // =========================================================================

    #[test]
    fn parse_with_explicit_compressed_format() {
        let vk = *default_signing_key().verifying_key();
        let pk = pubkey_compressed(&vk);
        let parsed = parse_public_key_with_format(ECDSA_KEY_FORMAT_COMPRESSED, &pk).unwrap();
        assert_eq!(parsed, vk);
    }

    #[test]
    fn parse_with_explicit_uncompressed_format() {
        let vk = *default_signing_key().verifying_key();
        let pk = pubkey_uncompressed_65(&vk);
        let parsed = parse_public_key_with_format(ECDSA_KEY_FORMAT_UNCOMPRESSED, &pk).unwrap();
        assert_eq!(parsed, vk);
    }

    #[test]
    fn parse_with_explicit_hybrid_format() {
        let vk = *default_signing_key().verifying_key();
        let pk = pubkey_hybrid(&vk);
        let parsed = parse_public_key_with_format(ECDSA_KEY_FORMAT_HYBRID, &pk).unwrap();
        assert_eq!(parsed, vk);
    }

    #[test]
    fn parse_with_explicit_raw_format() {
        let vk = *default_signing_key().verifying_key();
        let pk = pubkey_raw_64(&vk);
        let parsed = parse_public_key_with_format(ECDSA_KEY_FORMAT_RAW, &pk).unwrap();
        assert_eq!(parsed, vk);
    }

    #[test]
    fn parse_with_wrong_format_length_mismatch() {
        let vk = *default_signing_key().verifying_key();
        let pk = pubkey_compressed(&vk); // 33 bytes
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_RAW, &pk).is_err());
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_UNCOMPRESSED, &pk).is_err());
    }

    #[test]
    fn parse_with_unsupported_format_byte() {
        let vk = *default_signing_key().verifying_key();
        let pk = pubkey_compressed(&vk);
        assert!(parse_public_key_with_format(0x04, &pk).is_err());
        assert!(parse_public_key_with_format(0xFF, &pk).is_err());
    }

    // =========================================================================
    // INTEROP: same key, both paths
    // =========================================================================

    #[test]
    fn same_key_verifies_both_eth_and_btc() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();

        let (eth_sig, rid) = sign_ethereum(&sk, &msg);
        let eth_signature = EcdsaSignature::from_slice(&eth_sig[..64]).unwrap();
        let recovered = VerifyingKey::recover_from_prehash(&msg, &eth_signature, rid).unwrap();
        assert_eq!(recovered, vk);

        let btc_sig = sign_bitcoin(&sk, &msg);
        let btc_signature = EcdsaSignature::from_slice(&btc_sig).unwrap().normalize_s();
        assert!(vk.verify_prehash(&msg, &btc_signature).is_ok());
    }

    #[test]
    fn eth_sig_truncated_to_64_verifies_as_btc() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();

        let (eth_sig, _) = sign_ethereum(&sk, &msg);
        let truncated = EcdsaSignature::from_slice(&eth_sig[..64]).unwrap();
        let normalized = truncated.normalize_s();
        assert!(vk.verify_prehash(&msg, &normalized).is_ok());
    }

    // =========================================================================
    // PUBLIC KEY PARSING: INVALID INPUTS
    // =========================================================================

    #[test]
    fn parse_empty_fails() {
        assert!(parse_public_key(&[]).is_err());
    }

    #[test]
    fn parse_single_byte_fails() {
        // 1-byte input gets format Raw (len != 33/64/65), parse rejects len != 64
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_COMPRESSED, &[0x02]).is_err());
    }

    #[test]
    fn parse_compressed_wrong_prefix_0x04() {
        let sk = default_signing_key();
        let mut bytes = pubkey_compressed(sk.verifying_key());
        bytes[0] = 0x04;
        // Still 33 bytes, format tag = Compressed, but from_sec1_bytes rejects 0x04 in 33 bytes
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_COMPRESSED, &bytes).is_err());
    }

    #[test]
    fn parse_compressed_wrong_prefix_0x01() {
        let sk = default_signing_key();
        let mut bytes = pubkey_compressed(sk.verifying_key());
        bytes[0] = 0x01;
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_COMPRESSED, &bytes).is_err());
    }

    #[test]
    fn parse_uncompressed_wrong_prefix_0x05() {
        let sk = default_signing_key();
        let mut bytes = pubkey_uncompressed_65(sk.verifying_key());
        bytes[0] = 0x05;
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_UNCOMPRESSED, &bytes).is_err());
    }

    #[test]
    fn parse_uncompressed_truncated_60() {
        let sk = default_signing_key();
        let bytes = pubkey_uncompressed_65(sk.verifying_key());
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_UNCOMPRESSED, &bytes[..60]).is_err());
    }

    #[test]
    fn parse_raw_64_all_zeros() {
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_RAW, &[0u8; 64]).is_err());
    }

    #[test]
    fn parse_hybrid_wrong_length_33() {
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_HYBRID, &[0x06u8; 33]).is_err());
    }

    #[test]
    fn parse_garbage_17_bytes() {
        // Any format will reject 17 bytes
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_COMPRESSED, &[0xff; 17]).is_err());
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_RAW, &[0xff; 17]).is_err());
    }

    #[test]
    fn parse_garbage_100_bytes() {
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_COMPRESSED, &[0xab; 100]).is_err());
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_UNCOMPRESSED, &[0xab; 100]).is_err());
        assert!(parse_public_key_with_format(ECDSA_KEY_FORMAT_RAW, &[0xab; 100]).is_err());
    }

    #[test]
    fn from_sec1_rejects_infinity() {
        assert!(VerifyingKey::from_sec1_bytes(&[0x00]).is_err());
    }

    #[test]
    fn from_sec1_rejects_compressed_x_zero() {
        let mut bad = [0u8; 33];
        bad[0] = 0x02;
        assert!(VerifyingKey::from_sec1_bytes(&bad).is_err());
    }

    // =========================================================================
    // VERIFYING KEY EQUALITY (PartialEq)
    // =========================================================================

    #[test]
    fn vk_eq_same_key() {
        let sk = default_signing_key();
        assert_eq!(sk.verifying_key(), sk.verifying_key());
    }

    #[test]
    fn vk_ne_different_keys() {
        let a = default_signing_key();
        let b = alternate_signing_key();
        assert_ne!(a.verifying_key(), b.verifying_key());
    }

    #[test]
    fn vk_eq_from_compressed_vs_uncompressed() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();

        let from_c = VerifyingKey::from_sec1_bytes(&pubkey_compressed(&vk)).unwrap();
        let from_u = VerifyingKey::from_sec1_bytes(&pubkey_uncompressed_65(&vk)).unwrap();
        assert_eq!(from_c, from_u);
        assert_eq!(from_c, vk);
    }

    // =========================================================================
    // SCHNORR: BIP-340 hardcoded test vectors, verification only.
    // No secp256k1 signing used. Tests the exact same code path as
    // production: schnorr::verify(&sig, &msg, &xonly).
    // =========================================================================

    /// BIP-340 test vector 0: secret key = 3, simplest valid case.
    #[test]
    fn schnorr_bip340_vector_0() {
        let pk = hex_decode_32("F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9");
        let msg = hex_decode_32("0000000000000000000000000000000000000000000000000000000000000000");
        let sig = hex_decode_64(
            "E907831F80848D1069A5371B402410364BDF1C5F8307B0084C55F1CE2DCA8215\
             25F66A4A85EA8B71E482A74F382D2CE5EBEEE8FDB2172F477DF4900D310536C0",
        );

        let xonly = XOnlyPublicKey::from_byte_array(pk).unwrap();
        let signature = schnorr::Signature::from_byte_array(sig);
        assert!(schnorr::verify(&signature, &msg, &xonly).is_ok());
    }

    /// BIP-340 test vector 1.
    #[test]
    fn schnorr_bip340_vector_1() {
        let pk = hex_decode_32("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659");
        let msg = hex_decode_32("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89");
        let sig = hex_decode_64(
            "6896BD60EEAE296DB48A229FF71DFE071BDE413E6D43F917DC8DCF8C78DE3341\
             8906D11AC976ABCCB20B091292BFF4EA897EFCB639EA871CFA95F6DE339E4B0A",
        );

        let xonly = XOnlyPublicKey::from_byte_array(pk).unwrap();
        let signature = schnorr::Signature::from_byte_array(sig);
        assert!(schnorr::verify(&signature, &msg, &xonly).is_ok());
    }

    /// BIP-340 test vector 2.
    #[test]
    fn schnorr_bip340_vector_2() {
        let pk = hex_decode_32("DD308AFEC5777E13121FA72B9CC1B7CC0139715309B086C960E18FD969774EB8");
        let msg = hex_decode_32("7E2D58D8B3BCDF1ABADEC7829054F90DDA9805AAB56C77333024B9D0A508B75C");
        let sig = hex_decode_64(
            "5831AAEED7B44BB74E5EAB94BA9D4294C49BCF2A60728D8B4C200F50DD313C1B\
             AB745879A5AD954A72C45A91C3A51D3C7ADEA98D82F8481E0E1E03674A6F3FB7",
        );

        let xonly = XOnlyPublicKey::from_byte_array(pk).unwrap();
        let signature = schnorr::Signature::from_byte_array(sig);
        assert!(schnorr::verify(&signature, &msg, &xonly).is_ok());
    }

    /// BIP-340 test vector 3.
    #[test]
    fn schnorr_bip340_vector_3() {
        let pk = hex_decode_32("25D1DFF95105F5253C4022F628A996AD3A0D95FBF21D468A1B33F8C160D8F517");
        let msg = hex_decode_32("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
        let sig = hex_decode_64(
            "7EB0509757E246F19449885651611CB965ECC1A187DD51B64FDA1EDC9637D5EC\
             97582B9CB13DB3933705B32BA982AF5AF25FD78881EBB32771FC5922EFC66EA3",
        );

        let xonly = XOnlyPublicKey::from_byte_array(pk).unwrap();
        let signature = schnorr::Signature::from_byte_array(sig);
        assert!(schnorr::verify(&signature, &msg, &xonly).is_ok());
    }

    /// BIP-340 test vector 4: valid signature with low R.
    #[test]
    fn schnorr_bip340_vector_4_low_r() {
        let pk = hex_decode_32("D69C3509BB99E412E68B0FE8544E72837DFA30746D8BE2AA65975F29D22DC7B9");
        let msg = hex_decode_32("4DF3C3F68FCC83B27E9D42C90431A72499F17875C81A599B566C9889B9696703");
        let sig = hex_decode_64(
            "00000000000000000000003B78CE563F89A0ED9414F5AA28AD0D96D6795F9C63\
             76AFB1548AF603B3EB45C9F8207DEE1060CB71C04E80F593060B07D28308D7F4",
        );

        let xonly = XOnlyPublicKey::from_byte_array(pk).unwrap();
        let signature = schnorr::Signature::from_byte_array(sig);
        assert!(schnorr::verify(&signature, &msg, &xonly).is_ok());
    }

    /// BIP-340 test vector 5: public key NOT on curve (x exceeds field size).
    #[test]
    fn schnorr_bip340_vector_5_bad_pk() {
        let pk_bytes =
            hex_decode_32("EEFDEA4CDB677750A420FEE807EACF21EB9898AE79B9768766E4FAA04A2D4A34");
        assert!(XOnlyPublicKey::from_byte_array(pk_bytes).is_err());
    }

    /// BIP-340 test vector 6: has_even_y(R) is false, signature invalid.
    #[test]
    fn schnorr_bip340_vector_6_bad_r_parity() {
        let pk = hex_decode_32("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659");
        let msg = hex_decode_32("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89");
        let sig = hex_decode_64(
            "FFF97BD5755EEEA420453A14355235D382F6472F8568A18B2F057A1460297556\
             3CC27944640AC607CD107AE10923D9EF7A73C643E166BE5EBEAFA34B1AC553E2",
        );

        let xonly = XOnlyPublicKey::from_byte_array(pk).unwrap();
        let signature = schnorr::Signature::from_byte_array(sig);
        assert!(schnorr::verify(&signature, &msg, &xonly).is_err());
    }

    /// BIP-340 test vector 8: negated s value, signature invalid.
    #[test]
    fn schnorr_bip340_vector_8_negated_s() {
        let pk = hex_decode_32("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659");
        let msg = hex_decode_32("243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89");
        let sig = hex_decode_64(
            "6CFF5C3BA86C69EA4B7376F31A9BCB4F74C1976089B2D9963DA2E5543E177769\
             961764B3AA9B2FFCB6EF947B6887A226E8D7C93E00C5ED0C1834FF0D0C2E6DA6",
        );

        let xonly = XOnlyPublicKey::from_byte_array(pk).unwrap();
        let signature = schnorr::Signature::from_byte_array(sig);
        assert!(schnorr::verify(&signature, &msg, &xonly).is_err());
    }

    /// Schnorr: wrong message against valid vector 0 data.
    #[test]
    fn schnorr_wrong_message_fails() {
        let pk = hex_decode_32("F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9");
        let mut msg =
            hex_decode_32("0000000000000000000000000000000000000000000000000000000000000000");
        let sig = hex_decode_64(
            "E907831F80848D1069A5371B402410364BDF1C5F8307B0084C55F1CE2DCA8215\
             25F66A4A85EA8B71E482A74F382D2CE5EBEEE8FDB2172F477DF4900D310536C0",
        );

        msg[0] = 0xff; // corrupt message

        let xonly = XOnlyPublicKey::from_byte_array(pk).unwrap();
        let signature = schnorr::Signature::from_byte_array(sig);
        assert!(schnorr::verify(&signature, &msg, &xonly).is_err());
    }

    /// Schnorr: vector 0 signature against vector 1 public key.
    #[test]
    fn schnorr_wrong_key_fails() {
        let wrong_pk =
            hex_decode_32("DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659");
        let msg = hex_decode_32("0000000000000000000000000000000000000000000000000000000000000000");
        let sig = hex_decode_64(
            "E907831F80848D1069A5371B402410364BDF1C5F8307B0084C55F1CE2DCA8215\
             25F66A4A85EA8B71E482A74F382D2CE5EBEEE8FDB2172F477DF4900D310536C0",
        );

        let xonly = XOnlyPublicKey::from_byte_array(wrong_pk).unwrap();
        let signature = schnorr::Signature::from_byte_array(sig);
        assert!(schnorr::verify(&signature, &msg, &xonly).is_err());
    }

    /// Schnorr: x-only key roundtrip through serialize/parse.
    #[test]
    fn schnorr_xonly_roundtrip() {
        let pk = hex_decode_32("F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9");
        let xonly = XOnlyPublicKey::from_byte_array(pk).unwrap();
        let serialized = xonly.serialize();
        let rt = XOnlyPublicKey::from_byte_array(serialized).unwrap();
        assert_eq!(xonly, rt);
    }

    /// Schnorr: all-zero x-only key is invalid (point at infinity).
    #[test]
    fn schnorr_invalid_xonly_zeros() {
        assert!(XOnlyPublicKey::from_byte_array([0u8; 32]).is_err());
    }

    /// Schnorr: signature is exactly 64 bytes.
    #[test]
    fn schnorr_sig_is_64_bytes() {
        let sig = hex_decode_64(
            "E907831F80848D1069A5371B402410364BDF1C5F8307B0084C55F1CE2DCA8215\
             25F66A4A85EA8B71E482A74F382D2CE5EBEEE8FDB2172F477DF4900D310536C0",
        );
        assert_eq!(sig.len(), 64);
        let parsed = schnorr::Signature::from_byte_array(sig);
        assert_eq!(parsed.to_byte_array().len(), 64);
    }

    // =========================================================================
    // WIRE BUFFER LAYOUT (3-byte ECDSA header: type, subtype, format)
    // =========================================================================

    #[test]
    fn wire_ecdsa_ethereum_compressed() {
        let pk = pubkey_compressed(default_signing_key().verifying_key());
        let format = derive_format_tag(&pk);
        let mut buf = Vec::with_capacity(3 + pk.len());
        buf.push(0x00); // SignaturesMethods.ECDSA
        buf.push(0x00); // ECDSASubType.Ethereum
        buf.push(format);
        buf.extend_from_slice(&pk);
        assert_eq!(buf[0], 0x00);
        assert_eq!(buf[1], 0x00);
        assert_eq!(buf[2], ECDSA_KEY_FORMAT_COMPRESSED);
        assert_eq!(buf.len(), 36); // 3 header + 33 key
        assert!(buf[3] == 0x02 || buf[3] == 0x03);
    }

    #[test]
    fn wire_ecdsa_ethereum_raw() {
        let pk = pubkey_raw_64(default_signing_key().verifying_key());
        let format = derive_format_tag(&pk);
        let mut buf = Vec::with_capacity(3 + pk.len());
        buf.push(0x00);
        buf.push(0x00);
        buf.push(format);
        buf.extend_from_slice(&pk);
        assert_eq!(buf[2], ECDSA_KEY_FORMAT_RAW);
        assert_eq!(buf.len(), 67); // 3 header + 64 key
    }

    #[test]
    fn wire_ecdsa_bitcoin_compressed() {
        let pk = pubkey_compressed(default_signing_key().verifying_key());
        let format = derive_format_tag(&pk);
        let mut buf = Vec::with_capacity(3 + pk.len());
        buf.push(0x00); // SignaturesMethods.ECDSA
        buf.push(0x01); // ECDSASubType.Bitcoin
        buf.push(format);
        buf.extend_from_slice(&pk);
        assert_eq!(buf[0], 0x00);
        assert_eq!(buf[1], 0x01);
        assert_eq!(buf[2], ECDSA_KEY_FORMAT_COMPRESSED);
        assert_eq!(buf.len(), 36);
    }

    #[test]
    fn wire_ecdsa_bitcoin_uncompressed() {
        let pk = pubkey_uncompressed_65(default_signing_key().verifying_key());
        let format = derive_format_tag(&pk);
        let mut buf = Vec::with_capacity(3 + pk.len());
        buf.push(0x00);
        buf.push(0x01);
        buf.push(format);
        buf.extend_from_slice(&pk);
        assert_eq!(buf[2], ECDSA_KEY_FORMAT_UNCOMPRESSED);
        assert_eq!(buf.len(), 68); // 3 header + 65 key
    }

    #[test]
    fn wire_ecdsa_bitcoin_hybrid() {
        let pk = pubkey_hybrid(default_signing_key().verifying_key());
        let format = derive_format_tag(&pk);
        let mut buf = Vec::with_capacity(3 + pk.len());
        buf.push(0x00);
        buf.push(0x01);
        buf.push(format);
        buf.extend_from_slice(&pk);
        assert_eq!(buf[2], ECDSA_KEY_FORMAT_HYBRID);
        assert_eq!(buf.len(), 68); // 3 header + 65 key
    }

    #[test]
    fn wire_schnorr() {
        let pk = hex_decode_32("F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9");
        let mut buf = Vec::with_capacity(33);
        buf.push(0x01);
        buf.extend_from_slice(&pk);
        assert_eq!(buf[0], 0x01);
        assert_eq!(buf.len(), 33);
    }

    #[test]
    fn wire_mldsa() {
        let mut buf = Vec::new();
        buf.push(0x02);
        buf.push(0x00);
        buf.extend_from_slice(&[0xAA; 1312]);
        assert_eq!(buf[0], 0x02);
        assert_eq!(buf[1], 0x00);
        assert_eq!(buf.len(), 1314);
    }

    #[test]
    fn wire_unsupported_key_types() {
        for t in [0x03u8, 0x04, 0x10, 0xff] {
            assert!(t != 0x00 && t != 0x01 && t != 0x02);
        }
    }

    #[test]
    fn wire_unsupported_ecdsa_subtypes() {
        for s in [2u8, 3, 127, 255] {
            assert!(s != 0 && s != 1);
        }
    }

    #[test]
    fn wire_unsupported_ecdsa_key_formats() {
        for f in [0x04u8, 0x05, 0x10, 0xFF] {
            assert!(
                f != ECDSA_KEY_FORMAT_COMPRESSED
                    && f != ECDSA_KEY_FORMAT_UNCOMPRESSED
                    && f != ECDSA_KEY_FORMAT_HYBRID
                    && f != ECDSA_KEY_FORMAT_RAW
            );
        }
    }

    // =========================================================================
    // ECDSA SIGNATURE CONSTRUCTION EDGE CASES
    // =========================================================================

    #[test]
    fn ecdsa_sig_rejects_63_bytes() {
        assert!(EcdsaSignature::from_slice(&[0x01; 63]).is_err());
    }

    #[test]
    fn ecdsa_sig_rejects_65_bytes() {
        assert!(EcdsaSignature::from_slice(&[0x01; 65]).is_err());
    }

    #[test]
    fn ecdsa_sig_accepts_valid_64_bytes() {
        let sk = default_signing_key();
        let compact = sign_bitcoin(&sk, &test_message_hash());
        assert!(EcdsaSignature::from_slice(&compact).is_ok());
    }

    // =========================================================================
    // INTEGRATION TESTS: execute_inner with real Wasmer instance
    //
    // These tests exercise the complete host function path: memory reads,
    // header parsing, consensus flag enforcement, per-algorithm gas charging,
    // and end-to-end cryptographic verification through the wire protocol.
    // =========================================================================

    use crate::domain::runner::{
        ConsensusFlags, EnvironmentVariables, InstanceWrapper, WasmerRunner, MAX_PAGES,
    };
    use wasmer::{imports, AsStoreMut, Instance, Module};

    use super::{
        VerifySignatureImport, ECDSA_BITCOIN_VERIFY_GAS_COST, ECDSA_ETHEREUM_VERIFY_GAS_COST,
        ECDSA_SUBTYPE_BITCOIN, ECDSA_SUBTYPE_ETHEREUM, MLDSA_44_VERIFY_GAS_COST,
        MLDSA_65_VERIFY_GAS_COST, MLDSA_87_VERIFY_GAS_COST, SCHNORR_VERIFY_GAS_COST,
        STATIC_GAS_COST,
    };

    const TEST_GAS_LIMIT: u64 = 10_000_000_000;

    /// Creates a real Wasmer instance with memory and metering globals.
    /// The WAT module is compiled through the metering middleware so
    /// use_gas / get_remaining_gas work correctly.
    fn create_test_instance() -> (wasmer::Store, InstanceWrapper) {
        let mut store = WasmerRunner::create_engine(MAX_PAGES).unwrap();
        let wasm = wat::parse_str(
            "(module (memory (export \"memory\") 1) (func (export \"noop\") nop))",
        )
        .unwrap();
        let module = Module::new(&store, &wasm).unwrap();
        let instance = Instance::new(&mut store, &module, &imports! {}).unwrap();
        let wrapper = InstanceWrapper::new(instance, TEST_GAS_LIMIT);
        wrapper.set_remaining_gas(&mut store, TEST_GAS_LIMIT);
        (store, wrapper)
    }

    fn env_with_quantum_flag() -> EnvironmentVariables {
        EnvironmentVariables::new(
            &[0u8; 32],
            0,
            0,
            &[0u8; 32],
            &[0u8; 32],
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            &[],
            &[],
            &[],
            ConsensusFlags::UNSAFE_QUANTUM_SIGNATURES_ALLOWED,
        )
    }

    /// Writes data into the Wasmer instance's memory at the given offset.
    fn write_mem(store: &wasmer::Store, instance: &InstanceWrapper, offset: u64, data: &[u8]) {
        instance.write_memory(store, offset, data).unwrap();
    }

    /// Builds the ECDSA wire buffer: [type=0x00, subtype, format, ...key_material]
    fn build_ecdsa_pubkey_wire(subtype: u8, format: u8, key_material: &[u8]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(3 + key_material.len());
        buf.push(0x00);
        buf.push(subtype);
        buf.push(format);
        buf.extend_from_slice(key_material);
        buf
    }

    /// Builds the Schnorr wire buffer: [type=0x01, ...32-byte xonly pubkey]
    fn build_schnorr_pubkey_wire(xonly: &[u8; 32]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(33);
        buf.push(0x01);
        buf.extend_from_slice(xonly);
        buf
    }

    // -----------------------------------------------------------------
    // Consensus flag enforcement
    // -----------------------------------------------------------------

    #[test]
    fn execute_ecdsa_rejected_without_quantum_flag() {
        let (mut store, instance) = create_test_instance();
        let env = EnvironmentVariables::default(); // no flags

        // Write a valid ECDSA header (type=0, subtype=Ethereum)
        write_mem(&store, &instance, 0, &[0x00, ECDSA_SUBTYPE_ETHEREUM]);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            0,
            100,
            200,
        );
        let err = result.unwrap_err();
        assert!(
            err.message().contains("ECDSA"),
            "expected ECDSA rejection, got: {}",
            err.message()
        );
    }

    #[test]
    fn execute_schnorr_rejected_without_quantum_flag() {
        let (mut store, instance) = create_test_instance();
        let env = EnvironmentVariables::default();

        write_mem(&store, &instance, 0, &[0x01, 0x00]);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            0,
            100,
            200,
        );
        let err = result.unwrap_err();
        assert!(
            err.message().contains("Schnorr"),
            "expected Schnorr rejection, got: {}",
            err.message()
        );
    }

    #[test]
    fn execute_mldsa_allowed_without_quantum_flag() {
        let (mut store, instance) = create_test_instance();
        let env = EnvironmentVariables::default();

        // ML-DSA header: type=0x02, level=0x00 (ML-DSA-44)
        // Write enough fake data for the ML-DSA path to attempt reading.
        // It will fail during crypto verification, but the dispatch succeeds.
        write_mem(&store, &instance, 0, &[0x02, 0x00]);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            0,
            100,
            200,
        );
        // ML-DSA-44 should NOT be rejected by consensus flags.
        // It will fail later (reading key/sig) or return 0, but not with
        // a "not allowed by consensus rules" error.
        match &result {
            Err(e) => assert!(
                !e.message().contains("not allowed by consensus"),
                "ML-DSA should not be blocked by consensus flag, got: {}",
                e.message()
            ),
            Ok(v) => assert_eq!(*v, 0), // verification failed with garbage data, expected
        }
    }

    #[test]
    fn execute_missing_env_variables_returns_error() {
        let (mut store, instance) = create_test_instance();

        write_mem(&store, &instance, 0, &[0x00, 0x00]);

        let result = VerifySignatureImport::execute_inner(
            None, // no environment variables
            store.as_store_mut(),
            instance,
            0,
            100,
            200,
        );
        let err = result.unwrap_err();
        assert!(
            err.message().contains("Environment variables not found"),
            "got: {}",
            err.message()
        );
    }

    // -----------------------------------------------------------------
    // Header dispatch: unsupported types
    // -----------------------------------------------------------------

    #[test]
    fn execute_unsupported_key_type_returns_error() {
        let env = env_with_quantum_flag();

        for bad_type in [0x03u8, 0x04, 0x10, 0xFF] {
            let (mut store, instance) = create_test_instance();
            write_mem(&store, &instance, 0, &[bad_type, 0x00]);

            let result = VerifySignatureImport::execute_inner(
                Some(&env),
                store.as_store_mut(),
                instance,
                0,
                100,
                200,
            );
            let err = result.unwrap_err();
            assert!(
                err.message().contains("Unsupported public key type"),
                "type 0x{:02X}: {}",
                bad_type,
                err.message()
            );
        }
    }

    #[test]
    fn execute_unsupported_ecdsa_subtype_returns_error() {
        let env = env_with_quantum_flag();

        for bad_sub in [2u8, 3, 127, 255] {
            let (mut store, instance) = create_test_instance();
            write_mem(&store, &instance, 0, &[0x00, bad_sub]);

            let result = VerifySignatureImport::execute_inner(
                Some(&env),
                store.as_store_mut(),
                instance,
                0,
                100,
                200,
            );
            let err = result.unwrap_err();
            assert!(
                err.message().contains("Unsupported ECDSA sub-type"),
                "subtype {}: {}",
                bad_sub,
                err.message()
            );
        }
    }

    #[test]
    fn execute_invalid_mldsa_level_returns_error() {
        let env = env_with_quantum_flag();

        for bad_level in [3u8, 4, 127, 255] {
            let (mut store, instance) = create_test_instance();
            write_mem(&store, &instance, 0, &[0x02, bad_level]);

            let result = VerifySignatureImport::execute_inner(
                Some(&env),
                store.as_store_mut(),
                instance,
                0,
                100,
                200,
            );
            let err = result.unwrap_err();
            assert!(
                err.message().contains("MLDSA gas cost"),
                "level {}: {}",
                bad_level,
                err.message()
            );
        }
    }

    // -----------------------------------------------------------------
    // Gas metering
    // -----------------------------------------------------------------

    #[test]
    fn execute_charges_static_gas_on_unsupported_type() {
        let (mut store, instance) = create_test_instance();
        let env = env_with_quantum_flag();

        write_mem(&store, &instance, 0, &[0xFF, 0x00]);

        let _ = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance.clone(),
            0,
            100,
            200,
        );

        let used = instance.get_used_gas(&mut store);
        assert_eq!(used, STATIC_GAS_COST);
    }

    #[test]
    fn execute_charges_ecdsa_ethereum_gas() {
        let (mut store, instance) = create_test_instance();
        let env = env_with_quantum_flag();
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let (sig_wire, _) = sign_ethereum(&sk, &msg);

        let pk_wire = build_ecdsa_pubkey_wire(
            ECDSA_SUBTYPE_ETHEREUM,
            ECDSA_KEY_FORMAT_COMPRESSED,
            &pubkey_compressed(&vk),
        );

        let pk_ptr: u32 = 0;
        let sig_ptr: u32 = 256;
        let msg_ptr: u32 = 512;

        write_mem(&store, &instance, pk_ptr as u64, &pk_wire);
        write_mem(&store, &instance, sig_ptr as u64, &sig_wire);
        write_mem(&store, &instance, msg_ptr as u64, &msg);

        let _ = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance.clone(),
            pk_ptr,
            sig_ptr,
            msg_ptr,
        );

        let used = instance.get_used_gas(&mut store);
        assert_eq!(used, STATIC_GAS_COST + ECDSA_ETHEREUM_VERIFY_GAS_COST);
    }

    #[test]
    fn execute_charges_ecdsa_bitcoin_gas() {
        let (mut store, instance) = create_test_instance();
        let env = env_with_quantum_flag();
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);

        let pk_wire = build_ecdsa_pubkey_wire(
            ECDSA_SUBTYPE_BITCOIN,
            ECDSA_KEY_FORMAT_COMPRESSED,
            &pubkey_compressed(&vk),
        );

        let pk_ptr: u32 = 0;
        let sig_ptr: u32 = 256;
        let msg_ptr: u32 = 512;

        write_mem(&store, &instance, pk_ptr as u64, &pk_wire);
        write_mem(&store, &instance, sig_ptr as u64, &sig);
        write_mem(&store, &instance, msg_ptr as u64, &msg);

        let _ = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance.clone(),
            pk_ptr,
            sig_ptr,
            msg_ptr,
        );

        let used = instance.get_used_gas(&mut store);
        assert_eq!(used, STATIC_GAS_COST + ECDSA_BITCOIN_VERIFY_GAS_COST);
    }

    #[test]
    fn execute_charges_schnorr_gas() {
        let (mut store, instance) = create_test_instance();
        let env = env_with_quantum_flag();

        let pk = hex_decode_32(
            "F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9",
        );
        let msg = hex_decode_32(
            "0000000000000000000000000000000000000000000000000000000000000000",
        );
        let sig = hex_decode_64(
            "E907831F80848D1069A5371B402410364BDF1C5F8307B0084C55F1CE2DCA8215\
             25F66A4A85EA8B71E482A74F382D2CE5EBEEE8FDB2172F477DF4900D310536C0",
        );

        let pk_wire = build_schnorr_pubkey_wire(&pk);

        let pk_ptr: u32 = 0;
        let sig_ptr: u32 = 256;
        let msg_ptr: u32 = 512;

        write_mem(&store, &instance, pk_ptr as u64, &pk_wire);
        write_mem(&store, &instance, sig_ptr as u64, &sig);
        write_mem(&store, &instance, msg_ptr as u64, &msg);

        let _ = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance.clone(),
            pk_ptr,
            sig_ptr,
            msg_ptr,
        );

        let used = instance.get_used_gas(&mut store);
        assert_eq!(used, STATIC_GAS_COST + SCHNORR_VERIFY_GAS_COST);
    }

    #[test]
    fn execute_charges_mldsa_gas_per_level() {
        let expected = [
            (0u8, MLDSA_44_VERIFY_GAS_COST),
            (1u8, MLDSA_65_VERIFY_GAS_COST),
            (2u8, MLDSA_87_VERIFY_GAS_COST),
        ];

        let env = env_with_quantum_flag();

        for (level, expected_gas) in expected {
            let (mut store, instance) = create_test_instance();

            // ML-DSA header
            write_mem(&store, &instance, 0, &[0x02, level]);

            let _ = VerifySignatureImport::execute_inner(
                Some(&env),
                store.as_store_mut(),
                instance.clone(),
                0,
                100,
                200,
            );

            let used = instance.get_used_gas(&mut store);
            assert_eq!(
                used,
                STATIC_GAS_COST + expected_gas,
                "ML-DSA level {} gas mismatch",
                level,
            );
        }
    }

    // -----------------------------------------------------------------
    // End-to-end ECDSA Ethereum verification through memory
    // -----------------------------------------------------------------

    #[test]
    fn execute_ecdsa_ethereum_valid_signature_returns_1() {
        let (mut store, instance) = create_test_instance();
        let env = env_with_quantum_flag();
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let (sig_wire, _) = sign_ethereum(&sk, &msg);

        let pk_wire = build_ecdsa_pubkey_wire(
            ECDSA_SUBTYPE_ETHEREUM,
            ECDSA_KEY_FORMAT_COMPRESSED,
            &pubkey_compressed(&vk),
        );

        let pk_ptr: u32 = 0;
        let sig_ptr: u32 = 256;
        let msg_ptr: u32 = 512;

        write_mem(&store, &instance, pk_ptr as u64, &pk_wire);
        write_mem(&store, &instance, sig_ptr as u64, &sig_wire);
        write_mem(&store, &instance, msg_ptr as u64, &msg);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            pk_ptr,
            sig_ptr,
            msg_ptr,
        );
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn execute_ecdsa_ethereum_wrong_key_returns_0() {
        let (mut store, instance) = create_test_instance();
        let env = env_with_quantum_flag();
        let sk = default_signing_key();
        let other = alternate_signing_key();
        let msg = test_message_hash();
        let (sig_wire, _) = sign_ethereum(&sk, &msg);

        let pk_wire = build_ecdsa_pubkey_wire(
            ECDSA_SUBTYPE_ETHEREUM,
            ECDSA_KEY_FORMAT_COMPRESSED,
            &pubkey_compressed(other.verifying_key()),
        );

        let pk_ptr: u32 = 0;
        let sig_ptr: u32 = 256;
        let msg_ptr: u32 = 512;

        write_mem(&store, &instance, pk_ptr as u64, &pk_wire);
        write_mem(&store, &instance, sig_ptr as u64, &sig_wire);
        write_mem(&store, &instance, msg_ptr as u64, &msg);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            pk_ptr,
            sig_ptr,
            msg_ptr,
        );
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn execute_ecdsa_ethereum_all_key_formats() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let (sig_wire, _) = sign_ethereum(&sk, &msg);
        let env = env_with_quantum_flag();

        let formats: [(u8, Vec<u8>); 4] = [
            (ECDSA_KEY_FORMAT_COMPRESSED, pubkey_compressed(&vk)),
            (ECDSA_KEY_FORMAT_UNCOMPRESSED, pubkey_uncompressed_65(&vk)),
            (ECDSA_KEY_FORMAT_HYBRID, pubkey_hybrid(&vk)),
            (ECDSA_KEY_FORMAT_RAW, pubkey_raw_64(&vk)),
        ];

        for (fmt, key_bytes) in &formats {
            let (mut store, instance) = create_test_instance();

            let pk_wire = build_ecdsa_pubkey_wire(ECDSA_SUBTYPE_ETHEREUM, *fmt, key_bytes);

            write_mem(&store, &instance, 0, &pk_wire);
            write_mem(&store, &instance, 256, &sig_wire);
            write_mem(&store, &instance, 512, &msg);

            let result = VerifySignatureImport::execute_inner(
                Some(&env),
                store.as_store_mut(),
                instance,
                0,
                256,
                512,
            );
            assert_eq!(
                result.unwrap(),
                1,
                "Ethereum verify failed for format 0x{:02X}",
                fmt,
            );
        }
    }

    #[test]
    fn execute_ecdsa_ethereum_v27_v28_accepted() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let (mut sig_wire, rid) = sign_ethereum(&sk, &msg);
        let env = env_with_quantum_flag();

        // Remap v from 0/1 to 27/28 (EIP-155 style)
        sig_wire[64] = rid.to_byte() + 27;

        let pk_wire = build_ecdsa_pubkey_wire(
            ECDSA_SUBTYPE_ETHEREUM,
            ECDSA_KEY_FORMAT_COMPRESSED,
            &pubkey_compressed(&vk),
        );

        let (mut store, instance) = create_test_instance();
        write_mem(&store, &instance, 0, &pk_wire);
        write_mem(&store, &instance, 256, &sig_wire);
        write_mem(&store, &instance, 512, &msg);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            0,
            256,
            512,
        );
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn execute_ecdsa_ethereum_invalid_v_returns_0() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let (mut sig_wire, _) = sign_ethereum(&sk, &msg);
        let env = env_with_quantum_flag();

        sig_wire[64] = 5; // invalid v

        let pk_wire = build_ecdsa_pubkey_wire(
            ECDSA_SUBTYPE_ETHEREUM,
            ECDSA_KEY_FORMAT_COMPRESSED,
            &pubkey_compressed(&vk),
        );

        let (mut store, instance) = create_test_instance();
        write_mem(&store, &instance, 0, &pk_wire);
        write_mem(&store, &instance, 256, &sig_wire);
        write_mem(&store, &instance, 512, &msg);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            0,
            256,
            512,
        );
        assert_eq!(result.unwrap(), 0);
    }

    // -----------------------------------------------------------------
    // End-to-end ECDSA Bitcoin verification through memory
    // -----------------------------------------------------------------

    #[test]
    fn execute_ecdsa_bitcoin_valid_signature_returns_1() {
        let (mut store, instance) = create_test_instance();
        let env = env_with_quantum_flag();
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);

        let pk_wire = build_ecdsa_pubkey_wire(
            ECDSA_SUBTYPE_BITCOIN,
            ECDSA_KEY_FORMAT_COMPRESSED,
            &pubkey_compressed(&vk),
        );

        write_mem(&store, &instance, 0, &pk_wire);
        write_mem(&store, &instance, 256, &sig);
        write_mem(&store, &instance, 512, &msg);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            0,
            256,
            512,
        );
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn execute_ecdsa_bitcoin_wrong_message_returns_0() {
        let (mut store, instance) = create_test_instance();
        let env = env_with_quantum_flag();
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);

        let mut bad_msg = msg;
        bad_msg[0] ^= 0xFF;

        let pk_wire = build_ecdsa_pubkey_wire(
            ECDSA_SUBTYPE_BITCOIN,
            ECDSA_KEY_FORMAT_COMPRESSED,
            &pubkey_compressed(&vk),
        );

        write_mem(&store, &instance, 0, &pk_wire);
        write_mem(&store, &instance, 256, &sig);
        write_mem(&store, &instance, 512, &bad_msg);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            0,
            256,
            512,
        );
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn execute_ecdsa_bitcoin_all_key_formats() {
        let sk = default_signing_key();
        let vk = *sk.verifying_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);
        let env = env_with_quantum_flag();

        let formats: [(u8, Vec<u8>); 4] = [
            (ECDSA_KEY_FORMAT_COMPRESSED, pubkey_compressed(&vk)),
            (ECDSA_KEY_FORMAT_UNCOMPRESSED, pubkey_uncompressed_65(&vk)),
            (ECDSA_KEY_FORMAT_HYBRID, pubkey_hybrid(&vk)),
            (ECDSA_KEY_FORMAT_RAW, pubkey_raw_64(&vk)),
        ];

        for (fmt, key_bytes) in &formats {
            let (mut store, instance) = create_test_instance();

            let pk_wire = build_ecdsa_pubkey_wire(ECDSA_SUBTYPE_BITCOIN, *fmt, key_bytes);

            write_mem(&store, &instance, 0, &pk_wire);
            write_mem(&store, &instance, 256, &sig);
            write_mem(&store, &instance, 512, &msg);

            let result = VerifySignatureImport::execute_inner(
                Some(&env),
                store.as_store_mut(),
                instance,
                0,
                256,
                512,
            );
            assert_eq!(
                result.unwrap(),
                1,
                "Bitcoin verify failed for format 0x{:02X}",
                fmt,
            );
        }
    }

    #[test]
    fn execute_ecdsa_bitcoin_malformed_key_returns_0() {
        let (mut store, instance) = create_test_instance();
        let env = env_with_quantum_flag();
        let sk = default_signing_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);

        // Compressed key with all zeros (invalid point)
        let bad_key = vec![0x02; 33];
        let pk_wire = build_ecdsa_pubkey_wire(
            ECDSA_SUBTYPE_BITCOIN,
            ECDSA_KEY_FORMAT_COMPRESSED,
            &bad_key,
        );

        write_mem(&store, &instance, 0, &pk_wire);
        write_mem(&store, &instance, 256, &sig);
        write_mem(&store, &instance, 512, &msg);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            0,
            256,
            512,
        );
        // Should return Ok(0) not Err, matching Ethereum's behavior
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn execute_ecdsa_unsupported_key_format_returns_error() {
        let (mut store, instance) = create_test_instance();
        let env = env_with_quantum_flag();
        let sk = default_signing_key();
        let msg = test_message_hash();
        let sig = sign_bitcoin(&sk, &msg);

        // Format byte 0xFF is unsupported
        let pk_wire = build_ecdsa_pubkey_wire(ECDSA_SUBTYPE_BITCOIN, 0xFF, &[0u8; 33]);

        write_mem(&store, &instance, 0, &pk_wire);
        write_mem(&store, &instance, 256, &sig);
        write_mem(&store, &instance, 512, &msg);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            0,
            256,
            512,
        );
        // Bitcoin path now returns Ok(0) for key parse errors
        assert_eq!(result.unwrap(), 0);
    }

    // -----------------------------------------------------------------
    // End-to-end Schnorr verification through memory
    // -----------------------------------------------------------------

    #[test]
    fn execute_schnorr_valid_bip340_vector_returns_1() {
        let (mut store, instance) = create_test_instance();
        let env = env_with_quantum_flag();

        let pk = hex_decode_32(
            "F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9",
        );
        let msg = hex_decode_32(
            "0000000000000000000000000000000000000000000000000000000000000000",
        );
        let sig = hex_decode_64(
            "E907831F80848D1069A5371B402410364BDF1C5F8307B0084C55F1CE2DCA8215\
             25F66A4A85EA8B71E482A74F382D2CE5EBEEE8FDB2172F477DF4900D310536C0",
        );

        let pk_wire = build_schnorr_pubkey_wire(&pk);

        write_mem(&store, &instance, 0, &pk_wire);
        write_mem(&store, &instance, 256, &sig);
        write_mem(&store, &instance, 512, &msg);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            0,
            256,
            512,
        );
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn execute_schnorr_wrong_key_returns_0() {
        let (mut store, instance) = create_test_instance();
        let env = env_with_quantum_flag();

        // Use vector 1's public key with vector 0's signature/message
        let wrong_pk = hex_decode_32(
            "DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659",
        );
        let msg = hex_decode_32(
            "0000000000000000000000000000000000000000000000000000000000000000",
        );
        let sig = hex_decode_64(
            "E907831F80848D1069A5371B402410364BDF1C5F8307B0084C55F1CE2DCA8215\
             25F66A4A85EA8B71E482A74F382D2CE5EBEEE8FDB2172F477DF4900D310536C0",
        );

        let pk_wire = build_schnorr_pubkey_wire(&wrong_pk);

        write_mem(&store, &instance, 0, &pk_wire);
        write_mem(&store, &instance, 256, &sig);
        write_mem(&store, &instance, 512, &msg);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            0,
            256,
            512,
        );
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn execute_schnorr_wrong_message_returns_0() {
        let (mut store, instance) = create_test_instance();
        let env = env_with_quantum_flag();

        let pk = hex_decode_32(
            "F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9",
        );
        let mut msg = hex_decode_32(
            "0000000000000000000000000000000000000000000000000000000000000000",
        );
        msg[0] = 0xFF; // corrupt
        let sig = hex_decode_64(
            "E907831F80848D1069A5371B402410364BDF1C5F8307B0084C55F1CE2DCA8215\
             25F66A4A85EA8B71E482A74F382D2CE5EBEEE8FDB2172F477DF4900D310536C0",
        );

        let pk_wire = build_schnorr_pubkey_wire(&pk);

        write_mem(&store, &instance, 0, &pk_wire);
        write_mem(&store, &instance, 256, &sig);
        write_mem(&store, &instance, 512, &msg);

        let result = VerifySignatureImport::execute_inner(
            Some(&env),
            store.as_store_mut(),
            instance,
            0,
            256,
            512,
        );
        assert_eq!(result.unwrap(), 0);
    }

    // -----------------------------------------------------------------
    // Multiple deterministic keys through execute_inner
    // -----------------------------------------------------------------

    #[test]
    fn execute_ecdsa_ethereum_10_deterministic_keys() {
        let msg = test_message_hash();
        let env = env_with_quantum_flag();

        for idx in 1u8..=10 {
            let sk = signing_key_at_index(idx);
            let vk = *sk.verifying_key();
            let (sig_wire, _) = sign_ethereum(&sk, &msg);

            let pk_wire = build_ecdsa_pubkey_wire(
                ECDSA_SUBTYPE_ETHEREUM,
                ECDSA_KEY_FORMAT_COMPRESSED,
                &pubkey_compressed(&vk),
            );

            let (mut store, instance) = create_test_instance();
            write_mem(&store, &instance, 0, &pk_wire);
            write_mem(&store, &instance, 256, &sig_wire);
            write_mem(&store, &instance, 512, &msg);

            let result = VerifySignatureImport::execute_inner(
                Some(&env),
                store.as_store_mut(),
                instance,
                0,
                256,
                512,
            );
            assert_eq!(result.unwrap(), 1, "Ethereum verify failed at index {}", idx);
        }
    }

    #[test]
    fn execute_ecdsa_bitcoin_10_deterministic_keys() {
        let msg = test_message_hash();
        let env = env_with_quantum_flag();

        for idx in 1u8..=10 {
            let sk = signing_key_at_index(idx);
            let vk = *sk.verifying_key();
            let sig = sign_bitcoin(&sk, &msg);

            let pk_wire = build_ecdsa_pubkey_wire(
                ECDSA_SUBTYPE_BITCOIN,
                ECDSA_KEY_FORMAT_COMPRESSED,
                &pubkey_compressed(&vk),
            );

            let (mut store, instance) = create_test_instance();
            write_mem(&store, &instance, 0, &pk_wire);
            write_mem(&store, &instance, 256, &sig);
            write_mem(&store, &instance, 512, &msg);

            let result = VerifySignatureImport::execute_inner(
                Some(&env),
                store.as_store_mut(),
                instance,
                0,
                256,
                512,
            );
            assert_eq!(result.unwrap(), 1, "Bitcoin verify failed at index {}", idx);
        }
    }
}
