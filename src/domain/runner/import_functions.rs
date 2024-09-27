use bech32::{segwit, Hrp};
use ripemd::{Digest, Ripemd160};
use sha2::Sha256;
use tokio::runtime::Runtime;
use wasmer::{FunctionEnvMut, RuntimeError, StoreMut};

use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::{
    AbortData, CustomEnv, CALL_COST, DEPLOY_COST, ENCODE_ADDRESS_COST, LOAD_COST, SHA256_COST,
    STORE_COST,
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
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, store) = context.data_and_store_mut();
    external_import_with_param_and_return(
        env,
        store,
        &env.storage_load_external,
        ptr,
        LOAD_COST,
        &env.runtime,
    )
}

pub fn storage_store_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, store) = context.data_and_store_mut();
    external_import_with_param_and_return(
        env,
        store,
        &env.storage_store_external,
        ptr,
        STORE_COST,
        &env.runtime,
    )
}

/*pub fn storage_store_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();

    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    instance.use_gas(&mut store, STORE_COST);

    let data = AssemblyScript::read_buffer(&mut store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    let result = env.storage_store_external.execute(&data)?;

    let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
        .map_err(|_e| RuntimeError::new("Error writing buffer"))?;

    Ok(value as u32)
}*/

pub fn call_other_contract_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();

    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    instance.use_gas(&mut store, CALL_COST);

    let data = AssemblyScript::read_buffer(&store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    let result = &env
        .call_other_contract_external
        .execute(&data, &env.runtime)?;

    let call_execution_cost_bytes = &result[0..8];
    let response = &result[8..];

    let value = AssemblyScript::write_buffer(&mut store, &instance, &response, 13, 0)
        .map_err(|_e| RuntimeError::new("Error writing buffer"))?;

    let call_execution_cost = u64::from_le_bytes(call_execution_cost_bytes.try_into().unwrap());
    instance.use_gas(&mut store, call_execution_cost);

    Ok(value as u32)
}

pub fn deploy_from_address_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, store) = context.data_and_store_mut();
    external_import_with_param_and_return(
        env,
        store,
        &env.deploy_from_address_external,
        ptr,
        DEPLOY_COST,
        &env.runtime,
    )
}

pub fn encode_address_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();

    let instance = &env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    let network = &env.network;

    let data = AssemblyScript::read_buffer(&store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    // skip 4 bytes for length
    let data = data[4..].to_vec();
    if data.len() != 32 {
        return Err(RuntimeError::new(format!(
            "Invalid data length. Expected 32, got {}",
            data.len()
        )));
    }

    let mut ripemd = Ripemd160::new();
    ripemd.update(&data);
    let data = ripemd.finalize();

    let hrp = Hrp::parse(&network.contract_address_prefix()).expect("Valid hrp");
    let address = segwit::encode_v0(hrp, &data)
        .map_err(|e| RuntimeError::new(format!("Failed to encode address: {:?}", e)))?;

    let mut result = address.as_bytes().to_vec();

    result.push(0);

    let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
        .map_err(|_e| RuntimeError::new("Error writing buffer"))?;

    instance.use_gas(&mut store, ENCODE_ADDRESS_COST);

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
        .map_err(|_e| RuntimeError::new("Error writing buffer"))?;

    instance.use_gas(&mut store, SHA256_COST);

    Ok(value as u32)
}

fn sha256(data: &[u8]) -> Result<Vec<u8>, RuntimeError> {
    let hash = Sha256::digest(data);
    let hash_as_vec: Vec<u8> = hash.to_vec();

    Ok(hash_as_vec)
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
    let data = AssemblyScript::read_buffer(&store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    env.console_log_external.execute(&data)
}

fn external_import_with_param_and_return(
    env: &CustomEnv,
    mut store: StoreMut,
    external_function: &impl ExternalFunction,
    ptr: u32,
    gas_cost: u64,
    runtime: &Runtime,
) -> Result<u32, RuntimeError> {
    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    instance.use_gas(&mut store, gas_cost);

    let data = AssemblyScript::read_buffer(&mut store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    let result = external_function.execute(&data, runtime)?;

    let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
        .map_err(|_e| RuntimeError::new("Error writing buffer"))?;

    Ok(value as u32)
}

pub fn flush_events_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<(), RuntimeError> {
    let (env, store) = context.data_and_store_mut();

    let instance = &env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Memory not found"))?;
    let data = AssemblyScript::read_buffer(&store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    env.flush_events_external.execute(&data)
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
