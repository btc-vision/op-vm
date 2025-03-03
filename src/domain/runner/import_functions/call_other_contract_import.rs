use crate::domain::runner::call_result::CallResult;
use crate::domain::runner::CustomEnv;
use crate::interfaces::ExternalFunction;
use wasmer::{FunctionEnvMut, RuntimeError};

pub const STATIC_GAS_COST: u64 = 343_000_000;

#[derive(Default)]
pub struct CallOtherContractImport;

impl CallOtherContractImport {
    pub fn execute(
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

        instance.use_gas(&mut store, STATIC_GAS_COST);

        let gas_used = instance.get_gas_used(&mut store);

        let address = instance
            .read_memory(&store, address_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading address from memory"))?;

        let calldata = instance
            .read_memory(&store, calldata_ptr as u64, calldata_length as u64)
            .map_err(|_e| RuntimeError::new("Error reading calldata from memory"))?;

        let data = [
            gas_used.to_be_bytes().as_slice(),
            address.as_slice(),
            calldata_length.to_be_bytes().as_slice(),
            calldata.as_slice(),
        ]
        .concat();

        let result = &env
            .call_other_contract_external
            .execute(&data, &env.runtime)?;

        let call_execution_cost_bytes = result
            .get(0..8)
            .ok_or(RuntimeError::new("Invalid buffer"))?;

        let response = result
            .get(8..result.len())
            .ok_or(RuntimeError::new("Invalid buffer"))?;

        let bytes = call_execution_cost_bytes
            .try_into()
            .map_err(|_e| RuntimeError::new("Error converting bytes"))?;

        let call_execution_cost = u64::from_be_bytes(bytes);
        instance.use_gas(&mut store, call_execution_cost);

        env.last_call_result = CallResult::new(response);

        let result_length = response.len() as u32;
        let result_length_bytes = &result_length.to_be_bytes();

        instance
            .write_memory(&store, result_length_ptr as u64, result_length_bytes)
            .map_err(|_e| RuntimeError::new("Error writing call result to memory"))?;

        Ok(())
    }
}
