use crate::domain::runner::{
    verify_mldsa_internal, ConsensusFlags, CustomEnv, InstanceWrapper, MLDSAPublicKeyMetadata,
};
use secp256k1::schnorr::verify;
use secp256k1::{schnorr, XOnlyPublicKey};
use wasmer::{FunctionEnvMut, RuntimeError, StoreMut};

const STATIC_GAS_COST: u64 = 1_000_000;
const MLDSA_SIGNATURE_GAS_COST: u64 = 200_000_000;

#[derive(Default)]
pub struct VerifySignatureImport;

impl VerifySignatureImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        public_key_ptr: u32,
        signature_ptr: u32,
        message_ptr: u32,
    ) -> Result<u32, RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, STATIC_GAS_COST);

        let public_key_type_vec = instance
            .read_memory(&store, public_key_ptr as u64, 2)
            .map_err(|_e| RuntimeError::new("Error reading Schnorr public key type from memory"))?;

        let public_key_type = public_key_type_vec.get(0).cloned();
        if !public_key_type.is_some() {
            return Err(RuntimeError::new(
                "Error reading Schnorr public key type from memory",
            ));
        }

        if env.environment_variables.is_none() {
            return Err(RuntimeError::new("Environment variables not found"));
        }

        let env_variables = env
            .environment_variables
            .as_ref()
            .ok_or_else(|| RuntimeError::new("Environment variables not found"))?;

        let unsafe_quantum_signatures_allowed =
            env_variables.is_consensus_flag_set(ConsensusFlags::UNSAFE_QUANTUM_SIGNATURES_ALLOWED);

        let public_key_type = public_key_type.unwrap();
        match public_key_type {
            0 => {
                if !unsafe_quantum_signatures_allowed {
                    return Err(RuntimeError::new(
                        "Schnorr signature verification is not allowed by consensus rules",
                    ));
                }

                Self::verify_schnorr_signature(
                    store,
                    instance,
                    public_key_ptr,
                    signature_ptr,
                    message_ptr,
                )
            }
            1 => {
                let mldsa_level = public_key_type_vec.get(1).cloned();
                if !mldsa_level.is_some() {
                    return Err(RuntimeError::new(
                        "Error reading MLDSA public key level from memory",
                    ));
                }

                let mldsa_level = mldsa_level.as_ref().ok_or_else(|| {
                    RuntimeError::new("Error reading MLDSA public key level from memory")
                })?;

                Self::verify_mldsa_signature(
                    store,
                    instance,
                    public_key_ptr,
                    signature_ptr,
                    message_ptr,
                    mldsa_level.clone(),
                )
            }
            _ => Err(RuntimeError::new("Unsupported public key type")),
        }
    }

    fn verify_schnorr_signature(
        store: StoreMut,
        instance: InstanceWrapper,
        public_key_ptr: u32,
        signature_ptr: u32,
        message_ptr: u32,
    ) -> Result<u32, RuntimeError> {
        let public_key_bytes: [u8; 33] = instance
            .read_memory(&store, public_key_ptr as u64, 33)
            .map_err(|_e| RuntimeError::new("Error reading Schnorr public key from memory"))?
            .try_into()
            .map_err(|_e| RuntimeError::new("Error reading Schnorr public key from memory"))?;

        let public_key_bytes: [u8; 32] = public_key_bytes[1..33].try_into().map_err(|_e| {
            RuntimeError::new("Error converting Schnorr public key to xonly format")
        })?;

        let signature_bytes: [u8; 64] = instance
            .read_memory(&store, signature_ptr as u64, 64)
            .map_err(|_e| RuntimeError::new("Error reading Schnorr signature from memory"))?
            .try_into()
            .map_err(|_e| RuntimeError::new("Error reading Schnorr signature from memory"))?;

        let message_bytes = instance
            .read_memory(&store, message_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading Schnorr message from memory"))?;

        let xonly_public_key = XOnlyPublicKey::from_byte_array(public_key_bytes)
            .map_err(|e| RuntimeError::new(format!("Error converting public key: {}", e)))?;

        let signature = schnorr::Signature::from_byte_array(signature_bytes);
        let result = verify(&signature, &message_bytes, &xonly_public_key);
        let result = result.is_ok() as u32;

        Ok(result)
    }

    fn verify_mldsa_signature(
        mut store: StoreMut,
        instance: InstanceWrapper,
        public_key_ptr: u32,
        signature_ptr: u32,
        message_ptr: u32,
        mldsa_level: u8,
    ) -> Result<u32, RuntimeError> {
        instance.use_gas(&mut store, MLDSA_SIGNATURE_GAS_COST);

        let public_key_metadata =
            MLDSAPublicKeyMetadata::from_level(mldsa_level).ok_or_else(|| {
                RuntimeError::new("Error determining MLDSA public key metadata from level")
            })?;

        // Read public key (skip first 2 bytes: type and level)
        let public_key_bytes = instance
            .read_memory(
                &store,
                public_key_ptr as u64,
                2 + public_key_metadata.as_u64(),
            )
            .map_err(|e| RuntimeError::new(format!("Error reading ML-DSA public key: {:?}", e)))?;

        let public_key_bytes = &public_key_bytes[2..]; // Skip first 2 bytes

        // Read signature
        let signature_bytes = instance
            .read_memory(
                &store,
                signature_ptr as u64,
                public_key_metadata.signature_len() as u64,
            )
            .map_err(|e| RuntimeError::new(format!("Error reading ML-DSA signature: {:?}", e)))?;

        let message_bytes = instance
            .read_memory(&store, message_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading Schnorr message from memory"))?;

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
                    // log
                    eprintln!("ML-DSA signature verification panicked: {:?}", e);
                }

                Ok(0)
            }
        }
    }
}
