use crate::domain::runner::MAX_MEMORY_COPY_SIZE;
use wasmer::RuntimeError;

pub fn ensure_host_copy_length(length: u32, operation: &str) -> Result<(), RuntimeError> {
    ensure_host_copy_size(length as usize, operation)
}

pub fn ensure_host_copy_size(length: usize, operation: &str) -> Result<(), RuntimeError> {
    if length > MAX_MEMORY_COPY_SIZE as usize {
        return Err(RuntimeError::new(format!(
            "{} length exceeds maximum allowed size",
            operation
        )));
    }

    Ok(())
}
