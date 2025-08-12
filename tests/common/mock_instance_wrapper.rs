#![allow(dead_code)]

use op_vm::domain::runner::common::MemoryWriter;
use std::cell::RefCell;
use std::rc::Rc;
use wasmer::{AsStoreRef, MemoryAccessError, StoreMut};

/// Mock implementation of InstanceWrapper for testing
///
/// This mock tracks all memory operations and provides utilities for verification.
/// Memory is initialized with 0xFF to easily detect unwritten areas.
#[derive(Clone)]
pub struct MockInstanceWrapper {
    memory_contents: Rc<RefCell<Vec<u8>>>,
    write_calls: Rc<RefCell<Vec<MemoryWrite>>>,
    read_calls: Rc<RefCell<Vec<MemoryRead>>>,
    max_gas: u64,
    remaining_gas: Rc<RefCell<u64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MemoryWrite {
    pub offset: u64,
    pub data: Vec<u8>,
    pub timestamp: std::time::Instant,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MemoryRead {
    pub offset: u64,
    pub length: u64,
    pub timestamp: std::time::Instant,
}

impl MemoryWriter for MockInstanceWrapper {
    type Error = MemoryAccessError;

    fn write_memory(
        &self,
        _store: &impl AsStoreRef,
        offset: u64,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let mut memory = self.memory_contents.borrow_mut();
        let offset_usize = offset as usize;

        // Record the write call
        self.write_calls.borrow_mut().push(MemoryWrite {
            offset,
            data: data.to_vec(),
            timestamp: std::time::Instant::now(),
        });

        // Bounds check
        if offset_usize.saturating_add(data.len()) > memory.len() {
            return Err(MemoryAccessError::HeapOutOfBounds);
        }

        // Write data to memory
        memory[offset_usize..offset_usize + data.len()].copy_from_slice(data);
        Ok(())
    }
}

impl MockInstanceWrapper {
    /// Create a new mock instance with specified memory size and gas limit
    pub fn new(memory_size: usize, max_gas: u64) -> Self {
        Self {
            memory_contents: Rc::new(RefCell::new(vec![0xFF; memory_size])),
            write_calls: Rc::new(RefCell::new(Vec::new())),
            read_calls: Rc::new(RefCell::new(Vec::new())),
            max_gas,
            remaining_gas: Rc::new(RefCell::new(max_gas)),
        }
    }

    /// Create a new mock instance with memory initialized to specific values
    pub fn new_with_memory(memory: Vec<u8>, max_gas: u64) -> Self {
        let remaining_gas = max_gas;
        Self {
            memory_contents: Rc::new(RefCell::new(memory)),
            write_calls: Rc::new(RefCell::new(Vec::new())),
            read_calls: Rc::new(RefCell::new(Vec::new())),
            max_gas,
            remaining_gas: Rc::new(RefCell::new(remaining_gas)),
        }
    }

    /// Read data from memory (mimics InstanceWrapper::read_memory)
    pub fn read_memory(
        &self,
        _store: &StoreMut,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, MemoryAccessError> {
        let memory = self.memory_contents.borrow();
        let offset_usize = offset as usize;
        let length_usize = length as usize;

        // Record the read call
        self.read_calls.borrow_mut().push(MemoryRead {
            offset,
            length,
            timestamp: std::time::Instant::now(),
        });

        // Bounds check
        if offset_usize.saturating_add(length_usize) > memory.len() {
            return Err(MemoryAccessError::HeapOutOfBounds);
        }

        Ok(memory[offset_usize..offset_usize + length_usize].to_vec())
    }

    // Test helper methods

    /// Get the raw memory contents for inspection
    pub fn get_memory_contents(&self) -> Vec<u8> {
        self.memory_contents.borrow().clone()
    }

    /// Get a slice of memory contents
    pub fn get_memory_slice(&self, offset: usize, length: usize) -> Vec<u8> {
        let memory = self.memory_contents.borrow();
        memory[offset..offset + length].to_vec()
    }

    /// Get all write calls made to this instance
    pub fn get_write_calls(&self) -> Vec<MemoryWrite> {
        self.write_calls.borrow().clone()
    }

    /// Get all read calls made to this instance
    pub fn get_read_calls(&self) -> Vec<MemoryRead> {
        self.read_calls.borrow().clone()
    }

    /// Clear all recorded calls (useful for test isolation)
    pub fn clear_calls(&mut self) {
        self.write_calls.borrow_mut().clear();
        self.read_calls.borrow_mut().clear();
    }

    /// Reset memory to initial state (all 0xFF)
    pub fn reset_memory(&mut self) {
        let size = self.memory_contents.borrow().len();
        *self.memory_contents.borrow_mut() = vec![0xFF; size];
    }

    /// Set memory to specific values
    pub fn set_memory(&mut self, offset: usize, data: &[u8]) {
        let mut memory = self.memory_contents.borrow_mut();
        memory[offset..offset + data.len()].copy_from_slice(data);
    }

    /// Check if a memory region contains expected values
    pub fn assert_memory_equals(&self, offset: usize, expected: &[u8]) {
        let memory = self.memory_contents.borrow();
        let actual = &memory[offset..offset + expected.len()];
        assert_eq!(
            actual, expected,
            "Memory mismatch at offset {}: expected {:?}, got {:?}",
            offset, expected, actual
        );
    }

    /// Check if a memory region is uninitialized (all 0xFF)
    pub fn assert_memory_uninitialized(&self, offset: usize, length: usize) {
        let memory = self.memory_contents.borrow();
        let region = &memory[offset..offset + length];
        assert!(
            region.iter().all(|&b| b == 0xFF),
            "Memory at offset {} should be uninitialized (0xFF), but contains: {:?}",
            offset,
            region
        );
    }

    /// Get the number of write calls
    pub fn write_call_count(&self) -> usize {
        self.write_calls.borrow().len()
    }

    /// Get the number of read calls
    pub fn read_call_count(&self) -> usize {
        self.read_calls.borrow().len()
    }

    /// Verify that writes occurred in expected order and locations
    pub fn verify_write_sequence(&self, expected: &[(u64, &[u8])]) {
        let writes = self.write_calls.borrow();
        assert_eq!(
            writes.len(),
            expected.len(),
            "Expected {} writes, but got {}",
            expected.len(),
            writes.len()
        );

        for (i, (write, (expected_offset, expected_data))) in
            writes.iter().zip(expected.iter()).enumerate()
        {
            assert_eq!(
                write.offset, *expected_offset,
                "Write {} offset mismatch",
                i
            );
            assert_eq!(&write.data[..], *expected_data, "Write {} data mismatch", i);
        }
    }

    pub fn get_remaining_gas(&self, _store: &mut impl wasmer::AsStoreMut) -> u64 {
        *self.remaining_gas.borrow()
    }

    pub fn set_remaining_gas(&self, _store: &mut impl wasmer::AsStoreMut, gas: u64) {
        *self.remaining_gas.borrow_mut() = gas;
    }

    pub fn use_gas(&self, _store: &mut impl wasmer::AsStoreMut, gas: u64) {
        let mut remaining = self.remaining_gas.borrow_mut();
        *remaining = remaining.saturating_sub(gas);
    }

    pub fn get_used_gas(&self, _store: &mut impl wasmer::AsStoreMut) -> u64 {
        self.max_gas - *self.remaining_gas.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use op_vm::domain::runner::common::MemoryWriter;
    use wasmer::AsStoreMut;

    #[test]
    fn test_mock_basic_operations() {
        let mock = MockInstanceWrapper::new(100, 10000);
        let mut store = wasmer::Store::default();

        // Test write using the trait method
        assert!(<MockInstanceWrapper as MemoryWriter>::write_memory(
            &mock,
            &mut store.as_store_mut(),
            10,
            &[1, 2, 3]
        )
        .is_ok());
        assert_eq!(mock.write_call_count(), 1);

        // Test read
        let data = mock.read_memory(&mut store.as_store_mut(), 10, 3).unwrap();
        assert_eq!(data, vec![1, 2, 3]);
        assert_eq!(mock.read_call_count(), 1);

        // Test memory assertions
        mock.assert_memory_equals(10, &[1, 2, 3]);
        mock.assert_memory_uninitialized(0, 10);
        mock.assert_memory_uninitialized(13, 87);
    }

    #[test]
    fn test_mock_bounds_checking() {
        let mock = MockInstanceWrapper::new(10, 10000);
        let mut store = wasmer::Store::default();

        // Write beyond bounds
        assert!(<MockInstanceWrapper as MemoryWriter>::write_memory(
            &mock,
            &mut store.as_store_mut(),
            8,
            &[1, 2, 3]
        )
        .is_err());

        // Read beyond bounds
        assert!(mock.read_memory(&mut store.as_store_mut(), 5, 10).is_err());
    }

    #[test]
    fn test_mock_write_sequence_verification() {
        let mock = MockInstanceWrapper::new(100, 10000);
        let mut store = wasmer::Store::default();

        <MockInstanceWrapper as MemoryWriter>::write_memory(
            &mock,
            &mut store.as_store_mut(),
            10,
            &[1, 2],
        )
        .unwrap();
        <MockInstanceWrapper as MemoryWriter>::write_memory(
            &mock,
            &mut store.as_store_mut(),
            20,
            &[3, 4, 5],
        )
        .unwrap();

        mock.verify_write_sequence(&[(10, &[1, 2][..]), (20, &[3, 4, 5][..])]);
    }
}
