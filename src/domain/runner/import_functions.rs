use once_cell::sync::Lazy;
use ripemd::{Digest, Ripemd160};
use secp256k1::{schnorr, Secp256k1, XOnlyPublicKey};
use sha2::Sha256;
use std::cmp::min;
use wasmer::{FunctionEnvMut, RuntimeError, StoreMut};

use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::call_result::CallResult;
use crate::domain::runner::{
    exported_import_functions, AbortData, CustomEnv, InstanceWrapper, CALL_COST, CALL_RESULT_COST,
    DEPLOY_COST, EMIT_COST, INPUTS_COST, OUTPUTS_COST, RIMD160_STATIC_COST, RIMD160_WORD_COST,
    SCHNORR_VERIFICATION_STATIC_COST, SCHNORR_VERIFICATION_WORD_COST, SHA256_STATIC_COST,
    SHA256_WORD_COST, VALIDATE_BITCOIN_ADDRESS_STATIC_COST, VALIDATE_BITCOIN_ADDRESS_WORD_COST,
};
use crate::interfaces::ExternalFunction;

pub fn abort_import(
    mut env: FunctionEnvMut<CustomEnv>,
    message: u32,
    file_name: u32,
    line: u32,
    column: u32,
) -> Result<(), RuntimeError> {
    let data = env.data_mut();
    data.abort_data = Some(AbortData {
        message,
        file_name,
        line,
        column,
    });

    Err(RuntimeError::new("Execution aborted"))
}

pub fn storage_load_import(
    mut context: FunctionEnvMut<CustomEnv>,
    key_ptr: u32,
    result_ptr: u32,
) -> Result<(), RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();

    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    let data = instance.read_memory(&store, key_ptr as u64, 32)
        .map_err(|_e| RuntimeError::new("Error reading storage key from memory"))?;

    let mut cache = env
        .store_cache
        .lock()
        .map_err(|e| RuntimeError::new(format!("Error claiming store cache: {}", e)))?;

    // Get method
    let result = cache.get(
        &data
            .try_into()
            .map_err(|e| RuntimeError::new(format!("Cannot convert the pointer: {:?}", e)))?,
        |key| {
            Ok(env
                .storage_load_external
                .execute(&key, &env.runtime)?
                .try_into()
                .map_err(|e| RuntimeError::new(format!("Cannot map result to data: {:?}", e)))?)
        },
    )?;

    instance.use_gas(&mut store, result.gas_cost);

    instance.write_memory(&store, result_ptr as u64, &result.value)
        .map_err(|_e| RuntimeError::new("Error writing storage value to memory"))?;

    Ok(())
}

pub fn storage_store_import(
    mut context: FunctionEnvMut<CustomEnv>,
    key_ptr: u32,
    value_ptr: u32,
) -> Result<(), RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();

    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;
    let mut cache = env
        .store_cache
        .lock()
        .map_err(|e| RuntimeError::new(format!("Error claiming store cache: {}", e)))?;

    let key = instance.read_memory(&store, key_ptr as u64, 32)
        .map_err(|_e| RuntimeError::new("Error reading storage key from memory"))?;
    let value = instance.read_memory(&store, value_ptr as u64, 32)
        .map_err(|_e| RuntimeError::new("Error reading storage value from memory"))?;

    let data = [key.as_slice(), value.as_slice()].concat();

    let pointer = safe_slice(&data, 0, 32)
        .ok_or(RuntimeError::new("Invalid buffer"))?
        .to_vec();
    let value = safe_slice(&data, 32, 64)
        .ok_or(RuntimeError::new("Invalid buffer"))?
        .to_vec();

    let result = cache.set(
        pointer
            .try_into()
            .map_err(|e| RuntimeError::new(format!("Cannot convert the pointer: {:?}", e)))?,
        value
            .try_into()
            .map_err(|e| RuntimeError::new(format!("Cannot convert the pointer: {:?}", e)))?,
        |key| {
            Ok(env
                .storage_load_external
                .execute(&key, &env.runtime)?
                .try_into()
                .map_err(|e| RuntimeError::new(format!("Cannot map result to data: {:?}", e)))?)
        },
        |key, value| {
            env.storage_store_external
                .execute(
                    &key.iter().chain(value.iter()).cloned().collect::<Vec<u8>>(),
                    &env.runtime,
                )
                .map_err(|e| RuntimeError::new(format!("Cannot map result to data: {:?}", e)))?;
            Ok(())
        },
    )?;

    instance.use_gas(&mut store, result.gas_cost);
    if result.gas_refund > 0 {
        instance.refund_gas(&mut store, result.gas_refund as u64);
    } else if result.gas_refund < 0 && result.gas_refund > i64::MIN {
        instance.use_gas(&mut store, (-result.gas_refund) as u64);
    } else if result.gas_refund == i64::MIN {
        instance.use_gas(&mut store, u64::MAX);
    }

    Ok(())
}

pub fn call_other_contract_import(
    mut context: FunctionEnvMut<CustomEnv>,
    address_ptr: u32,
    calldata_ptr: u32,
    calldata_length: u32,
    result_length_ptr: u32,
) -> Result<(), RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();

    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    instance.use_gas(&mut store, CALL_COST);

    let address = instance
        .read_memory(&store, address_ptr as u64, 32)
        .map_err(|_e| RuntimeError::new("Error reading address from memory"))?;

    let calldata = instance
        .read_memory(&store, calldata_ptr as u64, calldata_length as u64)
        .map_err(|_e| RuntimeError::new("Error reading calldata from memory"))?;

    let data = [
        address.as_slice(),
        calldata_length.to_le_bytes().as_slice(),
        calldata.as_slice(),
    ]
    .concat();

    let result = &env
        .call_other_contract_external
        .execute(&data, &env.runtime)?;

    let call_execution_cost_bytes =
        safe_slice(&result, 0, 8).ok_or(RuntimeError::new("Invalid buffer"))?;
    let response =
        safe_slice(&result, 8, result.len()).ok_or(RuntimeError::new("Invalid buffer"))?;

    let bytes = call_execution_cost_bytes
        .try_into()
        .map_err(|_e| RuntimeError::new("Error converting bytes"))?;
    let call_execution_cost = u64::from_le_bytes(bytes);
    instance.use_gas(&mut store, call_execution_cost);

    env.last_call_result = CallResult::new(response);

    let result_length = response.len() as u32;
    let result_length_bytes = &result_length.to_be_bytes();

    instance
        .write_memory(&store, result_length_ptr as u64, result_length_bytes)
        .map_err(|_e| RuntimeError::new("Error writing call result to memory"))?;

    Ok(())
}

pub fn get_call_result_import(
    mut context: FunctionEnvMut<CustomEnv>,
    offset: u32,
    length: u32,
    result_ptr: u32,
) -> Result<(), RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();

    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    instance.use_gas(&mut store, CALL_RESULT_COST);

    let result_data = env.last_call_result.data.as_slice();

    write_call_result_data_slice_to_memory(
        &mut store,
        &instance,
        result_data,
        offset,
        length,
        result_ptr,
    )?;

    write_call_result_data_padding_to_memory(
        &mut store,
        instance,
        result_data,
        offset,
        length,
        result_ptr,
    )?;

    Ok(())
}

fn write_call_result_data_slice_to_memory(
    store: &mut StoreMut,
    instance: &InstanceWrapper,
    result_data: &[u8],
    offset: u32,
    length: u32,
    result_ptr: u32,
) -> Result<(), RuntimeError> {
    let result_data_sliced = slice_in_bounds(result_data, offset as usize, length as usize)
        .ok_or(RuntimeError::new("Error slicing call result"))?;

    if !result_data_sliced.is_empty() {
        instance
            .write_memory(&store, result_ptr as u64, result_data_sliced)
            .map_err(|_e| RuntimeError::new("Error writing call result to memory"))?;
    }
    Ok(())
}

fn write_call_result_data_padding_to_memory(
    store: &mut StoreMut,
    instance: InstanceWrapper,
    result_data: &[u8],
    offset: u32,
    length: u32,
    result_ptr: u32,
) -> Result<(), RuntimeError> {
    if result_data.len() < (offset + length) as usize {
        let zero_bytes_length = (offset + length) as usize - result_data.len();
        let zero_bytes_vec = vec![0; zero_bytes_length];
        let ptr = result_ptr as u64 + result_data.len() as u64;
        instance
            .write_memory(&store, ptr, &zero_bytes_vec)
            .map_err(|_e| RuntimeError::new("Error writing call result to memory"))?;
    }
    Ok(())
}

fn slice_in_bounds(data: &[u8], offset: usize, length: usize) -> Option<&[u8]> {
    let start = min(offset, data.len());
    let end = min(offset + length, data.len());
    safe_slice(data, start, end)
}

pub fn deploy_from_address_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();
    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    let data = AssemblyScript::read_buffer(&mut store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    instance.use_gas(&mut store, DEPLOY_COST);
    let result = &env.deploy_from_address_external.execute(&data, &env.runtime)?;
    let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
        .map_err(|e| RuntimeError::new(format!("Error writing buffer: {}", e)))?;

    Ok(value as u32)
}

pub fn emit_import(mut context: FunctionEnvMut<CustomEnv>, ptr: u32) -> Result<(), RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();
    let instance = &env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Memory not found"))?;

    instance.use_gas(&mut store, EMIT_COST);

    let data = AssemblyScript::read_buffer(&store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    env.emit_external.execute(&data, &env.runtime)
}

pub fn inputs_import(mut context: FunctionEnvMut<CustomEnv>) -> Result<u32, RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();

    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    instance.use_gas(&mut store, INPUTS_COST);

    let result = &env.inputs_external.execute(&env.runtime)?;
    let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
        .map_err(|e| RuntimeError::new(format!("Error writing buffer: {}", e)))?;

    Ok(value as u32)
}

pub fn outputs_import(mut context: FunctionEnvMut<CustomEnv>) -> Result<u32, RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();

    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    instance.use_gas(&mut store, OUTPUTS_COST);

    let result = &env.outputs_external.execute(&env.runtime)?;
    let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
        .map_err(|e| RuntimeError::new(format!("Error writing buffer: {}", e)))?;

    Ok(value as u32)
}

pub fn sha256_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();

    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    let data = AssemblyScript::read_buffer(&store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    let result = sha256(&data)?;

    let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
        .map_err(|e| RuntimeError::new(format!("Error writing buffer: {}", e)))?;

    instance.use_gas(
        &mut store,
        SHA256_STATIC_COST + ((data.len() + 31) / 32) as u64 * SHA256_WORD_COST,
    );

    Ok(value as u32)
}

fn sha256(data: &[u8]) -> Result<Vec<u8>, RuntimeError> {
    let hash = Sha256::digest(data);
    let hash_as_vec: Vec<u8> = hash.to_vec();

    Ok(hash_as_vec)
}

pub fn ripemd160_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();

    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    let data = AssemblyScript::read_buffer(&store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    let result = rimemd160(&data)?;

    let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
        .map_err(|e| RuntimeError::new(format!("Error writing buffer: {}", e)))?;

    instance.use_gas(
        &mut store,
        RIMD160_STATIC_COST + ((data.len() + 31) / 32) as u64 * RIMD160_WORD_COST,
    );

    Ok(value as u32)
}

fn rimemd160(data: &[u8]) -> Result<Vec<u8>, RuntimeError> {
    let mut ripemd = Ripemd160::new();
    ripemd.update(data);

    let hash = ripemd.finalize();
    let hash_as_vec: Vec<u8> = hash.to_vec();

    Ok(hash_as_vec)
}

// TODO: Add support for other blockchains
pub fn is_valid_bitcoin_address_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();

    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    let data = AssemblyScript::read_buffer(&store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;
    let data_len = data.len() as u64;

    let string_data = String::from_utf8(data)
        .map_err(|e| RuntimeError::new(format!("Error converting to string: {}", e)))?;
    let result = exported_import_functions::validate_bitcoin_address(&string_data, &env.network)
        .map_err(|e| RuntimeError::new(e))?;

    let result_vec_buffer = vec![result as u8];

    let value = AssemblyScript::write_buffer(&mut store, &instance, &result_vec_buffer, 13, 0)
        .map_err(|e| RuntimeError::new(format!("Error writing buffer: {}", e)))?;

    instance.use_gas(
        &mut store,
        VALIDATE_BITCOIN_ADDRESS_STATIC_COST + ((data_len + 31) / 32) * VALIDATE_BITCOIN_ADDRESS_WORD_COST,
    );

    Ok(value as u32)
}

pub fn verify_schnorr_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    static SECP: Lazy<Secp256k1<secp256k1::All>> = Lazy::new(|| Secp256k1::new());

    let (env, mut store) = context.data_and_store_mut();

    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    let data = AssemblyScript::read_buffer(&store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    let public_key_bytes = safe_slice(&data, 0, 32).ok_or(RuntimeError::new("Invalid buffer"))?;
    let signature_bytes =
        safe_slice(&data, 32, 96).ok_or(RuntimeError::new("Invalid signature"))?;
    let message_bytes = safe_slice(&data, 96, 128).ok_or(RuntimeError::new("Invalid message"))?;

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
        SCHNORR_VERIFICATION_STATIC_COST + ((data.len() + 31) / 32) as u64 * SCHNORR_VERIFICATION_WORD_COST,
    );

    Ok(value as u32)
}

pub fn console_log_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<(), RuntimeError> {
    let (env, store) = context.data_and_store_mut();
    let instance = &env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Memory not found"))?;

    let network = &env.network;
    if !&network.enable_debug() {
        return Err(RuntimeError::new("Contracts may not log this network"));
    }

    let data = AssemblyScript::read_buffer(&store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    env.console_log_external.execute(&data, &env.runtime)
}

fn safe_slice(vec: &[u8], start: usize, end: usize) -> Option<&[u8]> {
    vec.get(start..end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_hashes_number_correctly() {
        let data_to_hash = vec![9];
        let expected_hash =
            hex::decode("2b4c342f5433ebe591a1da77e013d1b72475562d48578dca8b84bac6651c3cb9")
                .unwrap();

        let result = sha256(&data_to_hash).unwrap();

        assert_eq!(result, expected_hash);
    }

    #[test]
    fn sha256_hashes_hex_data_correctly() {
        let data_to_hash = hex::decode("e3b0c44298fc1c149afbf4c8").unwrap().to_vec();
        let expected_hash =
            hex::decode("10dac508c2a7d7f0f3474c6ecc23f2a4d9ddbabec1009c4810f2ff677f4c1a83")
                .unwrap();

        let result = sha256(&data_to_hash).unwrap();

        assert_eq!(result, expected_hash);
    }
}
