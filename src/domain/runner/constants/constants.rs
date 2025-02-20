pub const MAX_PAGES: u32 = 128 * 4; // 1 page = 64KB, 32 MB.
pub const STACK_SIZE: usize = 1024 * 1024; // 1MB
pub const MAX_GAS_CONSTRUCTOR: u64 = 100_000_000; // only allow 100_000_000 gas for constructor
pub const PAGE_MEMORY_SIZE: u64 = 64 * 1024;
pub const MAX_MEMORY_SIZE: u64 = (MAX_PAGES as u64) * PAGE_MEMORY_SIZE;

/** Constraints */
pub const MAX_LENGTH_BUFFER_EXTERN: u32 = 2 * 1024 * 1024; // 4MB
