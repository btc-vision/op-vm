use wasmer::{FunctionEnvMut, Instance, RuntimeError, StoreMut};
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints, set_remaining_points};

use crate::domain::contract::AbortData;
use crate::domain::runner::CustomEnv;
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
    external_import_with_param_and_return(env, store, &env.call_other_contract_external, ptr, 500_000_000)
}

pub fn deploy_from_address_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, store) = context.data_and_store_mut();
    external_import_with_param_and_return(env, store, &env.deploy_from_address_external, ptr, 1_000_000_000)
}

pub fn encode_address_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, store) = context.data_and_store_mut();
    external_import_with_param_and_return(env, store, &env.encode_address_external, ptr, 300_000)
}

pub fn sha256_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<u32, RuntimeError> {
    let (env, mut store) = context.data_and_store_mut();

    let memory = env
        .memory
        .clone()
        .ok_or(RuntimeError::new("Memory not found"))?;
    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    let view = memory.view(&store);

    let data = env
        .read_buffer(&view, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    let result = env.sha256(&data)?;

    let value = env
        .write_buffer(&instance, &mut store, &result, 13, 0)
        .map_err(|_e| RuntimeError::new("Error writing buffer"))?;

    subtract_gas(store, &instance, 300_000);

    Ok(value as u32)
}

pub fn console_log_import(
    mut context: FunctionEnvMut<CustomEnv>,
    ptr: u32,
) -> Result<(), RuntimeError> {
    let (env, store) = context.data_and_store_mut();
    let memory = env
        .memory
        .clone()
        .ok_or(RuntimeError::new("Memory not found"))?;
    let view = memory.view(&store);

    let data = env
        .read_buffer(&view, ptr)
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
    let memory = env
        .memory
        .clone()
        .ok_or(RuntimeError::new("Memory not found"))?;
    let instance = env
        .instance
        .clone()
        .ok_or(RuntimeError::new("Instance not found"))?;

    let view = memory.view(&store);

    let data = env
        .read_buffer(&view, ptr)
        .map_err(|_e| RuntimeError::new("Error lifting typed array"))?;

    let result = external_function.execute(&data)?;

    let value = env
        .write_buffer(&instance, &mut store, &result, 13, 0)
        .map_err(|_e| RuntimeError::new("Error writing buffer"))?;

    subtract_gas(store, &instance, gas_cost);

    Ok(value as u32)
}

fn subtract_gas(mut store: StoreMut, instance: &Instance, gas_cost: u64) {
    let points_before = get_remaining_points(&mut store, &instance);
    let gas_before = match points_before {
        MeteringPoints::Remaining(points) => points,
        MeteringPoints::Exhausted => 0,
    };

    let gas_after = if gas_before <= gas_cost {
        0
    } else {
        gas_before - gas_cost
    };

    set_remaining_points(&mut store, &instance, gas_after);
}
