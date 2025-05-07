use std::panic;

use domain::runner::{NEW_STORAGE_SLOT_GAS_COST, UPDATED_STORAGE_SLOT_GAS_COST};
use interfaces::ContractManager;
use neon::{prelude::*, types::JsBigInt};

mod application;
mod domain;
mod interfaces;

pub const PROTOTYPE: &str = "prototype";
pub const INNER: &str = "inner";

/// Plain Rust struct to hold the request data

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    //cx.export_function("callJsFromRust", call_js_from_rust)?;

    panic::set_hook(Box::new(|e| {
        println!("Panic occurred: {:?}", e);
    }));

    println!("Hello from main");

    // Create JS functions
    let new_storag_slot_gas_cost = JsBigInt::from_u64(&mut cx, NEW_STORAGE_SLOT_GAS_COST);
    let updated_storage_slot_gas_cost = JsBigInt::from_u64(&mut cx, UPDATED_STORAGE_SLOT_GAS_COST);

    let contract_manager = JsFunction::new(&mut cx, ContractManager::js_constructor)?;
    let contract_manager_reserve_id = JsFunction::new(&mut cx, ContractManager::js_reserve_id)?;
    let contract_manager_instantiate = JsFunction::new(&mut cx, ContractManager::js_instantiate)?;
    let contract_manager_destroy_contract =
        JsFunction::new(&mut cx, ContractManager::js_destroy_contract)?;
    let contract_manager_destroy = JsFunction::new(&mut cx, ContractManager::js_destroy)?;
    let contract_manager_destroy_cache =
        JsFunction::new(&mut cx, ContractManager::js_destroy_cache)?;
    let contract_manager_destroy_all = JsFunction::new(&mut cx, ContractManager::js_destroy_all)?;
    let contract_manager_use_gas = JsFunction::new(&mut cx, ContractManager::js_use_gas)?;
    let contract_manager_get_exit_data =
        JsFunction::new(&mut cx, ContractManager::js_get_exit_data)?;
    let contract_manager_get_used_gas = JsFunction::new(&mut cx, ContractManager::js_get_used_gas)?;
    let contract_manager_write_memory = JsFunction::new(&mut cx, ContractManager::js_write_memory)?;
    let contract_manager_read_memory = JsFunction::new(&mut cx, ContractManager::js_read_memory)?;
    let contract_manager_set_environment_variable =
        JsFunction::new(&mut cx, ContractManager::js_set_environment_variables)?;
    let contract_manager_on_deploy = JsFunction::new(&mut cx, ContractManager::js_on_deploy)?;
    let contract_manager_execute = JsFunction::new(&mut cx, ContractManager::js_execute)?;
    let contract_manager_length = JsFunction::new(&mut cx, ContractManager::js_length)?;
    let contract_manager_clear = JsFunction::new(&mut cx, ContractManager::js_clear)?;

    // Register functions to prototype
    let contract_manager_prototype: Handle<JsObject> =
        contract_manager.prop(&mut cx, PROTOTYPE).get()?;
    contract_manager_prototype
        .prop(&mut cx, "reserveId")
        .set(contract_manager_reserve_id)?;
    contract_manager_prototype
        .prop(&mut cx, "instantiate")
        .set(contract_manager_instantiate)?;
    contract_manager_prototype
        .prop(&mut cx, "destroyContract")
        .set(contract_manager_destroy_contract)?;
    contract_manager_prototype
        .prop(&mut cx, "destroy")
        .set(contract_manager_destroy)?;
    contract_manager_prototype
        .prop(&mut cx, "destroyCache")
        .set(contract_manager_destroy_cache)?;
    contract_manager_prototype
        .prop(&mut cx, "destroyAll")
        .set(contract_manager_destroy_all)?;
    contract_manager_prototype
        .prop(&mut cx, "useGas")
        .set(contract_manager_use_gas)?;
    contract_manager_prototype
        .prop(&mut cx, "getExitData")
        .set(contract_manager_get_exit_data)?;
    contract_manager_prototype
        .prop(&mut cx, "getUsedGas")
        .set(contract_manager_get_used_gas)?;
    contract_manager_prototype
        .prop(&mut cx, "writeMemory")
        .set(contract_manager_write_memory)?;
    contract_manager_prototype
        .prop(&mut cx, "readMemory")
        .set(contract_manager_read_memory)?;
    contract_manager_prototype
        .prop(&mut cx, "setEnvironmentVariables")
        .set(contract_manager_set_environment_variable)?;
    contract_manager_prototype
        .prop(&mut cx, "onDeploy")
        .set(contract_manager_on_deploy)?;
    contract_manager_prototype
        .prop(&mut cx, "execute")
        .set(contract_manager_execute)?;
    contract_manager_prototype
        .prop(&mut cx, "length")
        .set(contract_manager_length)?;
    contract_manager_prototype
        .prop(&mut cx, "clear")
        .set(contract_manager_clear)?;

    cx.export_value("ContractManager", contract_manager)?;

    cx.export_value("NEW_STORAGE_SLOT_GAS_COST", new_storag_slot_gas_cost)?;
    cx.export_value(
        "UPDATED_STORAGE_SLOT_GAS_COST",
        updated_storage_slot_gas_cost,
    )?;

    println!("Rust initialized");

    Ok(())
}
