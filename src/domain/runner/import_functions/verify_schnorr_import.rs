use crate::domain::runner::CustomEnv;
use once_cell::sync::Lazy;
use secp256k1::{schnorr, Secp256k1, XOnlyPublicKey};
use wasmer::{FunctionEnvMut, RuntimeError};

pub const STATIC_GAS_COST: u64 = 100_000_000;

#[derive(Default)]
pub struct VerifySchnorrImport;

impl VerifySchnorrImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        public_key_ptr: u32,
        signature_ptr: u32,
        message_ptr: u32,
    ) -> Result<u32, RuntimeError> {
        static SECP: Lazy<Secp256k1<secp256k1::All>> = Lazy::new(|| Secp256k1::new());

        let (env, mut store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let public_key_bytes: [u8; 32] = instance
            .read_memory(&store, public_key_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading Schnorr public key from memory"))?
            .try_into()
            .map_err(|_e| RuntimeError::new("Error reading Schnorr public key from memory"))?;

        let signature_bytes: [u8; 64] = instance
            .read_memory(&store, signature_ptr as u64, 64)
            .map_err(|_e| RuntimeError::new("Error reading Schnorr signature from memory"))?
            .try_into()
            .map_err(|_e| RuntimeError::new("Error reading Schnorr signature from memory"))?;

        let message_bytes = instance
            .read_memory(&store, message_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading Schnorr message from memory"))?;

        let xonly_public_key = XOnlyPublicKey::from_byte_array(&public_key_bytes)
            .map_err(|e| RuntimeError::new(format!("Error converting public key: {}", e)))?;

        let signature = schnorr::Signature::from_byte_array(signature_bytes);
        let result = SECP.verify_schnorr(&signature, &message_bytes, &xonly_public_key);
        let result = result.is_ok() as u32;

        instance.use_gas(&mut store, STATIC_GAS_COST);

        Ok(result)
    }
}
