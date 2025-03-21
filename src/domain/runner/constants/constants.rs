pub const MAX_PAGES: u32 = 128 * 4; // 1 page = 64KB, 32 MB.
pub const STACK_SIZE: usize = 1024 * 1024; // 1MB
pub const PAGE_MEMORY_SIZE: u64 = 64 * 1024;
pub const MAX_MEMORY_SIZE: u64 = (MAX_PAGES as u64) * PAGE_MEMORY_SIZE;
pub const MAX_GAS_CONSTRUCTOR: u64 = 50_000_000;