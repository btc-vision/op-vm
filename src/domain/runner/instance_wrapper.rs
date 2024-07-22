use wasmer::{
    AsStoreMut, AsStoreRef, ExportError, Function, Instance, Memory, MemoryAccessError,
    RuntimeError, StoreMut, Value,
};
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints, set_remaining_points};

#[derive(Clone)]
pub struct InstanceWrapper {
    instance: Instance,
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
        let result = export.call(store, params)?;

        Ok(result)
    }

    pub fn read_memory(
        &self,
        store: &(impl AsStoreRef + ?Sized),
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, RuntimeError> {
        let memory = Self::get_memory(&self.instance);
        let view = memory.view(store);

        let mut buffer: Vec<u8> = vec![0; length as usize];
        view.read(offset, &mut buffer)?;

        Ok(buffer)
    }

    pub fn write_memory(
        &self,
        store: &(impl AsStoreRef + ?Sized),
        offset: u64,
        data: &[u8],
    ) -> Result<(), MemoryAccessError> {
        let memory = Self::get_memory(&self.instance);
        let view = memory.view(store);
        view.write(offset, data)
    }

    pub fn subtract_gas(&self, mut store: StoreMut, gas_cost: u64) {
        let gas_before = self.get_remaining_gas(&mut store);

        let gas_after = if gas_before <= gas_cost {
            0
        } else {
            gas_before - gas_cost
        };

        self.set_remaining_gas(&mut store, gas_after);
    }

    pub fn get_remaining_gas(&self, store: &mut impl AsStoreMut) -> u64 {
        let remaining_points = get_remaining_points(store, &self.instance);
        match remaining_points {
            MeteringPoints::Remaining(remaining) => remaining,
            MeteringPoints::Exhausted => 0,
        }
    }

    pub fn set_remaining_gas(&self, store: &mut impl AsStoreMut, gas: u64) {
        set_remaining_points(store, &self.instance, gas);
    }

    fn get_memory(instance: &Instance) -> &Memory {
        instance.exports.get_memory("memory").unwrap()
    }

    fn get_function<'a>(
        instance: &'a Instance,
        function: &str,
    ) -> Result<&'a Function, ExportError> {
        instance.exports.get_function(function)
    }
}