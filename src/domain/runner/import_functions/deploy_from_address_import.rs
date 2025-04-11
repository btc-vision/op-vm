use crate::domain::runner::CustomEnv;
use crate::interfaces::ExternalFunction;
use wasmer::{FunctionEnvMut, RuntimeError};

const STATIC_GAS_COST: u64 = 320_000_000;
const GAS_COST_PER_CONTRACT_BYTES: u64 = 2_000_000;

#[derive(Default)]
pub struct DeployFromAddressImport;

impl DeployFromAddressImport {
    pub fn execute(
        mut context: FunctionEnvMut<CustomEnv>,
        origin_address_ptr: u32,
        salt_ptr: u32,
        calldata_ptr: u32,
        calldata_length: u32,
        result_address_ptr: u32,
    ) -> Result<u32, RuntimeError> {
        let (env, mut store) = context.data_and_store_mut();

        if env.is_running_start_function {
            return Err(RuntimeError::new(
                "Cannot deploy contract in start function",
            ));
        }

        let instance = env
            .instance
            .clone()
            .ok_or(RuntimeError::new("Instance not found"))?;

        instance.use_gas(&mut store, STATIC_GAS_COST);

        let origin_address = instance
            .read_memory(&store, origin_address_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading address from memory"))?;

        let salt = instance
            .read_memory(&store, salt_ptr as u64, 32)
            .map_err(|_e| RuntimeError::new("Error reading salt from memory"))?;

        let calldata = instance
            .read_memory(&store, calldata_ptr as u64, calldata_length as u64)
            .map_err(|_e| RuntimeError::new("Error reading calldata from memory"))?;

        let gas_used = instance.get_used_gas(&mut store);

        let data = [
            gas_used.to_be_bytes().as_slice(),
            origin_address.as_slice(),
            salt.as_slice(),
            calldata.as_slice(),
        ]
            .concat();

        let result = &env
            .deploy_from_address_external
            .execute(&data, &env.runtime)?;

        let (result_address, result_remainder) = result.split_first_chunk::<32>().ok_or(
            RuntimeError::new("Invalid data received for 'Deploy from address'"),
        )?;

        let (bytecode_length_bytes, result_remainder) = result_remainder
            .split_first_chunk::<4>()
            .ok_or(RuntimeError::new(
                "Invalid data received for 'Deploy from address'",
            ))?;

        // Use deployment gas for bytecode
        let bytecode_length = u32::from_be_bytes(*bytecode_length_bytes);
        let gas_for_bytecode = bytecode_length as u64 * GAS_COST_PER_CONTRACT_BYTES;

        instance.use_gas(&mut store, gas_for_bytecode);

        let (call_execution_cost_bytes, result_remainder) = result_remainder
            .split_first_chunk::<8>()
            .ok_or(RuntimeError::new(
                "Invalid data received for 'Deploy from address'",
            ))?;

        let (exit_status_bytes, result_remainder) =
            result_remainder
                .split_first_chunk::<4>()
                .ok_or(RuntimeError::new(
                    "Invalid data received for 'Deploy from address'",
                ))?;

        let call_execution_cost = u64::from_be_bytes(*call_execution_cost_bytes);
        let exit_status = u32::from_be_bytes(*exit_status_bytes);

        instance.use_gas(&mut store, call_execution_cost);

        // Result from onDeploy
        let _exit_data = result_remainder.get(0..result_remainder.len()).ok_or(RuntimeError::new(
            "Invalid data received for 'Deploy from address'",
        ))?;

        instance
            .write_memory(&store, result_address_ptr as u64, result_address)
            .map_err(|_e| RuntimeError::new("Error writing contract address to memory"))?;

        Ok(exit_status)
    }
}
