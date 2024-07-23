use bech32::{Hrp, segwit};
use ripemd::{Digest, Ripemd160};
use wasmer::{FunctionEnvMut, RuntimeError, StoreMut};

use crate::domain::assembly_script::AssemblyScript;
use crate::domain::runner::{AbortData, CustomEnv};
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

    return Err(RuntimeError::new("Execution aborted"));
}

pub fn storage_load_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, store) = context.data_and_store_mut();
    external_import_with_param_and_return(env, store, &env.storage_load_external, ptr, 21_000_000)
}

pub fn storage_store_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, store) = context.data_and_store_mut();
    external_import_with_param_and_return(env, store, &env.storage_store_external, ptr, 221_000_000)
}

pub fn call_other_contract_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, store) = context.data_and_store_mut();
    external_import_with_param_and_return(
        env,
        store,
        &env.call_other_contract_external,
        ptr,
        500_000_000,
    )
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
        1_000_000_000,
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

    let hrp = Hrp::parse(&network.address_prefix()).expect("Valid hrp");
    let address = segwit::encode_v0(hrp, &data)
        .map_err(|e| RuntimeError::new(format!("Failed to encode address: {:?}", e)))?;

    let mut result = address.as_bytes().to_vec();

    result.push(0);

    let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
        .map_err(|_e| RuntimeError::new("Error writing buffer"))?;

    instance.use_gas(&mut store, 300_000);

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

    let result = env.sha256(&data)?;

    let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
        .map_err(|_e| RuntimeError::new("Error writing buffer"))?;

    instance.use_gas(&mut store, 300_000);

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
) -> Result<u32, RuntimeError> {
    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    let data = AssemblyScript::read_buffer(&mut store, &instance, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    let result = external_function.execute(&data)?;

    let value = AssemblyScript::write_buffer(&mut store, &instance, &result, 13, 0)
        .map_err(|_e| RuntimeError::new("Error writing buffer"))?;

    instance.use_gas(&mut store, gas_cost);

    Ok(value as u32)
}
