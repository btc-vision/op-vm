use crate::domain::runner::call_result::CallResult;
use crate::domain::runner::CustomEnv;
use crate::interfaces::ExternalFunction;
use wasmer::{FunctionEnvMut, RuntimeError};

const ADDRESS_ACCESS_GAS_COST: u64 = 26_000_000;

#[derive(Default)]
pub struct CallOtherContractImport;

impl CallOtherContractImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        address_ptr: u32,
        calldata_ptr: u32,
        calldata_length: u32,
        result_length_ptr: u32,
    ) -> Result<u32, RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new("Cannot call contract in start function"));
        }

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, ADDRESS_ACCESS_GAS_COST);

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

        let (call_execution_cost_bytes, result_remainder) = result
            .split_first_chunk::<8>()
            .ok_or(RuntimeError::new("Invalid data received for 'Call contract'"))?;
        let (exit_status_bytes, result_remainder) = result_remainder
            .split_first_chunk::<4>()
            .ok_or(RuntimeError::new("Invalid data received for 'Call contract'"))?;
        let response = result_remainder
            .get(0..result_remainder.len())
            .ok_or(RuntimeError::new("Invalid data received for 'Call contract'"))?;

        let call_execution_cost = u64::from_be_bytes(*call_execution_cost_bytes);
        let exit_status = u32::from_be_bytes(*exit_status_bytes);

        instance.use_gas(&mut store, call_execution_cost);

        env.last_call_result = CallResult::new(response);

        let result_length = response.len() as u32;
        let result_length_bytes = &result_length.to_be_bytes();

        instance
            .write_memory(&store, result_length_ptr as u64, result_length_bytes)
            .map_err(|_e| RuntimeError::new("Error writing call result to memory"))?;

        Ok(exit_status)
    }
}
