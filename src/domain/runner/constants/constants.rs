pub const MAX_PAGES: u32 = 128 * 4; // 1 page = 64KB, 32 MB.
pub const STACK_SIZE: usize = 1024 * 1024; // 1MB
pub const PAGE_MEMORY_SIZE: u64 = 64 * 1024;
pub const MAX_MEMORY_SIZE: u64 = (MAX_PAGES as u64) * PAGE_MEMORY_SIZE;
pub const MAX_GAS_WASM_INIT: u64 = 20_000_000;

pub const COLD_ADDRESS_ACCESS_GAS_COST: u64 = 26_000_000;
pub const WARM_ADDRESS_ACCESS_GAS_COST: u64 = 1_000_000;
