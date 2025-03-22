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
        let data = [origin_address.as_slice(), salt.as_slice()].concat();

        let result = &env
            .deploy_from_address_external
            .execute(&data, &env.runtime)?;

        let (result_address, result_remainder) = result.split_first_chunk::<32>().ok_or(
            RuntimeError::new("Invalid data received for 'Deploy from address'"),
        )?;
        let bytecode_length_bytes =
            result_remainder
                .first_chunk::<4>()
                .ok_or(RuntimeError::new(
                    "Invalid data received for 'Deploy from address'",
                ))?;

        let bytecode_length = u32::from_be_bytes(*bytecode_length_bytes);

        instance.use_gas(
            &mut store,
            bytecode_length as u64 * GAS_COST_PER_CONTRACT_BYTES,
        );

        instance
            .write_memory(&store, result_address_ptr as u64, result_address)
            .map_err(|_e| RuntimeError::new("Error writing contract address to memory"))?;

        if result_address == &[0; 32] {
            return Ok(1);
        }

        Ok(0)
    }
}
