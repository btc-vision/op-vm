use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::CustomEnv;
use once_cell::sync::Lazy;
use secp256k1::{schnorr, Secp256k1, XOnlyPublicKey};
use wasmer::{FunctionEnvMut, RuntimeError};

pub const STATIC_GAS_COST: u64 = 100_000_000;
pub const GAS_COST_PER_WORD: u64 = 120_000;

#[derive(Default)]
pub struct VerifySchnorrImport;

impl VerifySchnorrImport {
    pub fn execute(mut context: FunctionEnvMut<CustomEnv>, ptr: u32) -> Result<u32, RuntimeError> {
        static SECP: Lazy<Secp256k1<secp256k1::All>> = Lazy::new(|| Secp256k1::new());

        let (env, mut store) = context.data_and_store_mut();

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        let data = AssemblyScript::read_buffer(&store, &instance, ptr)
            .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

        let public_key_bytes = data.get(0..32).ok_or(RuntimeError::new("Invalid buffer"))?;
        let signature_bytes = data
            .get(32..96)
            .ok_or(RuntimeError::new("Invalid signature"))?;
        let message_bytes = data
            .get(96..128)
            .ok_or(RuntimeError::new("Invalid message"))?;

        let pub_key_bytes: [u8; 32] = public_key_bytes
            .try_into()
            .map_err(|e| RuntimeError::new(format!("Error converting bytes: {}", e)))?;

        let signature_bytes: [u8; 64] = signature_bytes
            .try_into()
            .map_err(|e| RuntimeError::new(format!("Error converting bytes: {}", e)))?;

        let xonly_public_key = XOnlyPublicKey::from_byte_array(&pub_key_bytes)
            .map_err(|e| RuntimeError::new(format!("Error converting public key: {}", e)))?;

        let signature = schnorr::Signature::from_byte_array(signature_bytes);
        let valid = SECP.verify_schnorr(&signature, &message_bytes, &xonly_public_key);

        let result = if valid.is_ok() { vec![1] } else { vec![0] };

        let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
            .map_err(|e| RuntimeError::new(format!("Error writing buffer: {}", e)))?;

        instance.use_gas(
            &mut store,
            STATIC_GAS_COST
                + ((data.len() + 31) / 32) as u64 * GAS_COST_PER_WORD,
        );

        Ok(value as u32)
    }
}
