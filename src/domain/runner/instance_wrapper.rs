use crate::domain::runner::MAX_MEMORY_SIZE;
use thiserror::Error;
use wasmer::{
    AsStoreMut, AsStoreRef, ExportError, Function, Instance, Memory, MemoryAccessError, Value,
};
use wasmer_middlewares::metering::{get_remaining_points, set_remaining_points, MeteringPoints};

#[derive(Clone)]
pub struct InstanceWrapper {
    instance: Instance,
}

#[derive(Clone, Copy, Debug, Error)]
#[non_exhaustive]
pub enum ExtendedMemoryAccessError {
    #[error("Memory access error: {0}")]
    Base(MemoryAccessError),

    #[error("Unable to get memory")]
    UnableToGetMemory,

    #[error("Out of memory")]
    Unknown,
}

impl InstanceWrapper {
    pub fn new(instance: Instance) -> Self {
        Self { instance }
    }

    pub fn call(
        &self,
        store: &mut impl AsStoreMut,
        function: &str,
        params: &[Value],
    ) -> anyhow::Result<Box<[Value]>> {
        let export = Self::get_function(&self.instance, function)?;
        println!("!!! Calling function: {:?} !!!", function);
        let result = export.call(store, params)?;
        println!("!!! Function call result: {:?} !!!", result);

        Ok(result)
    }

    pub fn is_out_of_memory(
        &self,
        store: &(impl AsStoreRef + ?Sized),
    ) -> Result<bool, ExtendedMemoryAccessError> {
        let memory = Self::get_memory(&self.instance)?;
        let view = memory.view(store);
        let size = view.data_size();

        Ok(MAX_MEMORY_SIZE <= size)
    }

    pub fn read_memory(
        &self,
        store: &(impl AsStoreRef + ?Sized),
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, ExtendedMemoryAccessError> {
        let memory = Self::get_memory(&self.instance)?;
        let view = memory.view(store);

        let mut buffer: Vec<u8> = vec![0; length as usize];
        view.read(offset, &mut buffer)
            .map_err(|e| ExtendedMemoryAccessError::Base(e))?;

        Ok(buffer)
    }

    pub fn read_memory_u8(
        &self,
        store: &(impl AsStoreRef + ?Sized),
        offset: u64,
    ) -> Result<u8, ExtendedMemoryAccessError> {
        let memory = Self::get_memory(&self.instance)?;
        let view = memory.view(store);
        view.read_u8(offset)
            .map_err(|e| ExtendedMemoryAccessError::Base(e))
    }

    pub fn write_memory(
        &self,
        store: &(impl AsStoreRef + ?Sized),
        offset: u64,
        data: &[u8],
    ) -> Result<(), ExtendedMemoryAccessError> {
        let memory = Self::get_memory(&self.instance)?;
        let view = memory.view(store);
        view.write(offset, data)
            .map_err(|e| ExtendedMemoryAccessError::Base(e))
    }

    pub fn use_gas(&self, store: &mut impl AsStoreMut, gas_cost: u64) {
        let gas_before = self.get_remaining_gas(store);

        let gas_after = if gas_before <= gas_cost {
            0
        } else {
            gas_before - gas_cost
        };

        self.set_remaining_gas(store, gas_after);
    }

    pub fn refund_gas(&self, store: &mut impl AsStoreMut, refund: u64) {
        let gas_before = self.get_remaining_gas(store);

        // check for overflow
        if gas_before > u64::MAX - refund {
            log::error!("Gas refund overflow detected. Setting remaining gas to u64::MAX");

            self.set_remaining_gas(store, u64::MAX);
            return;
        }

        let gas_after = gas_before + refund;
        self.set_remaining_gas(store, gas_after);
    }

    pub fn get_remaining_gas(&self, store: &mut impl AsStoreMut) -> u64 {
        println!("!!! Getting remaining gas !!!");
        let remaining_points = get_remaining_points(store, &self.instance);
        println!("!!! Remaining points: {:?} !!!", remaining_points);
        match remaining_points {
            MeteringPoints::Remaining(remaining) => remaining,
            MeteringPoints::Exhausted => 0,
        }
    }

    pub fn set_remaining_gas(&self, store: &mut impl AsStoreMut, gas: u64) {
        set_remaining_points(store, &self.instance, gas);
    }

    fn get_memory(instance: &Instance) -> Result<&Memory, ExtendedMemoryAccessError> {
        // TODO: Restore error state?
        instance
            .exports
            .get_memory("memory")
            .map_err(|_| ExtendedMemoryAccessError::UnableToGetMemory)
    }

    fn get_function<'a>(
        instance: &'a Instance,
        function: &str,
    ) -> Result<&'a Function, ExportError> {
        instance.exports.get_function(function)
    }
}
