use crate::domain::runner::MAX_MEMORY_SIZE;
use crate::domain::vm::{get_remaining_points, set_remaining_points, MeteringPoints};
use thiserror::Error;
use wasmer::{AsStoreMut, AsStoreRef, ExportError, Function, Instance, Memory, MemoryAccessError};
use wasmer_types::Pages;

use crate::domain::runner::common::MemoryWriter;

#[derive(Clone)]
pub struct InstanceWrapper {
    pub(crate) instance: Instance,
    max_gas: u64,
}

#[derive(Clone, Copy, Debug, Error)]
#[non_exhaustive]
pub enum ExtendedMemoryAccessError {
    #[error("Memory access error: {0}")]
    Base(MemoryAccessError),

    #[error("Unable to get memory")]
    UnableToGetMemory,
}

impl InstanceWrapper {
    pub fn new(instance: Instance, max_gas: u64) -> Self {
        Self {
            instance,
            max_gas,
        }
    }

    pub fn is_out_of_memory(
        &self,
        store: &(impl AsStoreRef + ?Sized),
    ) -> Result<bool, ExtendedMemoryAccessError> {
        let memory = Self::get_memory(&self.instance)?;
        Ok(MAX_MEMORY_SIZE <= memory.view(store).data_size())
    }

    pub fn read_memory(
        &self,
        store: &(impl AsStoreRef + ?Sized),
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, ExtendedMemoryAccessError> {
        let memory = Self::get_memory(&self.instance)?;
        let mut buffer = vec![0u8; length as usize];
        memory
            .view(store)
            .read(offset, &mut buffer)
            .map_err(ExtendedMemoryAccessError::Base)?;
        Ok(buffer)
    }

    pub fn write_memory(
        &self,
        store: &(impl AsStoreRef + ?Sized),
        offset: u64,
        data: &[u8],
    ) -> Result<(), ExtendedMemoryAccessError> {
        let memory = Self::get_memory(&self.instance)?;
        memory
            .view(store)
            .write(offset, data)
            .map_err(ExtendedMemoryAccessError::Base)
    }

    pub fn get_memory_size(
        &self,
        store: &(impl AsStoreRef + ?Sized),
    ) -> Result<Pages, ExtendedMemoryAccessError> {
        Ok(Self::get_memory(&self.instance)?.view(store).size())
    }

    pub(crate) fn get_memory(instance: &Instance) -> Result<&Memory, ExtendedMemoryAccessError> {
        instance
            .exports
            .get_memory("memory")
            .map_err(|_| ExtendedMemoryAccessError::UnableToGetMemory)
    }

    pub fn use_gas(&self, store: &mut impl AsStoreMut, gas_cost: u64) {
        let remaining = self.get_remaining_gas(store);
        self.set_remaining_gas(store, remaining.saturating_sub(gas_cost));
    }

    pub fn get_remaining_gas(&self, store: &mut impl AsStoreMut) -> u64 {
        match get_remaining_points(store, &self.instance) {
            MeteringPoints::Remaining(r) => r,
            MeteringPoints::Exhausted => 0,
        }
    }

    pub fn get_used_gas(&self, store: &mut impl AsStoreMut) -> u64 {
        self.max_gas - self.get_remaining_gas(store)
    }

    pub fn set_remaining_gas(&self, store: &mut impl AsStoreMut, gas: u64) {
        set_remaining_points(store, &self.instance, gas);
    }

    pub fn get_function(&self, name: &str) -> Result<&Function, ExportError> {
        self.instance.exports.get_function(name)
    }
}

impl MemoryWriter for InstanceWrapper {
    type Error = ExtendedMemoryAccessError;

    fn write_memory(
        &self,
        store: &impl AsStoreRef,
        offset: u64,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        self.write_memory(store, offset, data)
    }
}
