use napi::Error;
use sha2::{Digest, Sha256};
use wasmer::{Memory, MemoryView, RuntimeError};

use crate::domain::contract::AbortData;
use crate::domain::runner::InstanceWrapper;
use crate::interfaces::{
    CallOtherContractExternalFunction, ConsoleLogExternalFunction,
    DeployFromAddressExternalFunction, EncodeAddressExternalFunction, StorageLoadExternalFunction,
    StorageStoreExternalFunction,
};

pub struct CustomEnv {
    pub instance: Option<InstanceWrapper>,
    pub memory: Option<Memory>,
    pub abort_data: Option<AbortData>,
    pub storage_load_external: StorageLoadExternalFunction,
    pub storage_store_external: StorageStoreExternalFunction,
    pub call_other_contract_external: CallOtherContractExternalFunction,
    pub deploy_from_address_external: DeployFromAddressExternalFunction,
    pub console_log_external: ConsoleLogExternalFunction,
    pub encode_address_external: EncodeAddressExternalFunction,
}

impl CustomEnv {
    pub fn new(
        storage_load_external: StorageLoadExternalFunction,
        storage_store_external: StorageStoreExternalFunction,
        call_other_contract_external: CallOtherContractExternalFunction,
        deploy_from_address_external: DeployFromAddressExternalFunction,
        console_log_external: ConsoleLogExternalFunction,
        encode_address_external: EncodeAddressExternalFunction,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            instance: None,
            memory: None,
            abort_data: None,
            storage_load_external,
            storage_store_external,
            call_other_contract_external,
            deploy_from_address_external,
            console_log_external,
            encode_address_external,
        })
    }

    pub fn sha256(&self, data: &[u8]) -> Result<Vec<u8>, RuntimeError> {
        let hash = Sha256::digest(data);
        let hash_as_vec: Vec<u8> = hash.to_vec();

        Ok(hash_as_vec)
    }

    pub fn read_pointer(
        &self,
        memory_view: &MemoryView,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, RuntimeError> {
        let mut buffer: Vec<u8> = vec![0; length as usize];
        for i in 0..length {
            let byte = memory_view.read_u8(offset + i);

            // check for error
            if byte.is_err() {
                return Err(RuntimeError::new(format!(
                    "Failed to read byte at offset {} length {}. Error: {:?}",
                    offset + i,
                    length,
                    byte.unwrap_err()
                )));
            }

            buffer[i as usize] = byte.unwrap();
        }

        Ok(buffer)
    }

    pub fn bytes_to_u32_le(&self, bytes: Vec<u8>, offset: u32) -> u32 {
        let mut result = 0;
        for i in 0..4 {
            result |= (bytes[i + offset as usize] as u32) << (i * 8);
        }
        result
    }

    pub fn read_buffer(&self, memory_view: &MemoryView, offset: u32) -> Result<Vec<u8>, Error> {
        let pointer = self.read_pointer(memory_view, (offset + 4) as u64, 8);
        let pointer_buffer = pointer.map_err(|e| Error::from_reason(e.to_string()))?;

        let data_offset = self.bytes_to_u32_le(pointer_buffer.clone(), 0);
        let length = self.bytes_to_u32_le(pointer_buffer, 4);

        let result = self
            .read_pointer(memory_view, data_offset as u64, length as u64)
            .map_err(|e| Error::from_reason(e.to_string()))?;

        Ok(result)
    }
}
