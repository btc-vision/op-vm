pub mod mock_instance_wrapper;

pub use mock_instance_wrapper::{MemoryRead, MemoryWrite, MockInstanceWrapper};
use op_vm::domain::runner::{WasmerRunner, MAX_PAGES};
use wasmer_compiler::CompilerConfig;

/// Create a mock Store for testing
///
/// Creates a minimal Wasmer store suitable for testing memory operations.
pub fn create_mock_store() -> wasmer::Store {
    let store = WasmerRunner::create_engine(MAX_PAGES).unwrap();
    store
}

/// Common test constants
pub mod test_constants {
    /// Default memory size for tests (1MB)
    pub const DEFAULT_MEMORY_SIZE: usize = 1024 * 1024;

    /// Default gas limit for tests
    pub const DEFAULT_GAS_LIMIT: u64 = 1_000_000;
}

/// Helper function to create a standard test environment
pub fn setup_test_env() -> (MockInstanceWrapper, wasmer::Store) {
    let instance = MockInstanceWrapper::new(
        test_constants::DEFAULT_MEMORY_SIZE,
        test_constants::DEFAULT_GAS_LIMIT,
    );
    let store = create_mock_store();
    (instance, store)
}

/// Helper function to create a test environment with custom parameters
pub fn setup_test_env_with_params(
    memory_size: usize,
    gas_limit: u64,
) -> (MockInstanceWrapper, wasmer::Store) {
    let instance = MockInstanceWrapper::new(memory_size, gas_limit);
    let store = create_mock_store();
    (instance, store)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_module_imports() {
        // Verify that the module exports work correctly
        let _mock = MockInstanceWrapper::new(100, 1000);
        let _memory_read = MemoryRead {
            offset: 0,
            length: 10,
            timestamp: std::time::Instant::now(),
        };
        let _memory_write = MemoryWrite {
            offset: 0,
            data: vec![1, 2, 3],
            timestamp: std::time::Instant::now(),
        };
    }

    #[test]
    fn test_create_mock_store() {
        // Test that we can create a mock store
        let _store = create_mock_store();
    }

    #[test]
    fn test_setup_helpers() {
        // Test standard environment setup
        let (instance, _store) = setup_test_env();
        assert_eq!(instance.write_call_count(), 0);
        assert_eq!(instance.read_call_count(), 0);

        // Test custom environment setup
        let (instance2, _store2) = setup_test_env_with_params(2048, 5000);
        assert_eq!(instance2.write_call_count(), 0);
    }
}
