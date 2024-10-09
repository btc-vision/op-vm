pub const MAX_PAGES: u32 = 128 * 4; // 1 page = 64KB, 32 MB.
pub const STACK_SIZE: usize = 1024 * 1024; // 1MB
pub const MAX_GAS_CONSTRUCTOR: u64 = 100_000_000; // only allow 100_000_000 gas for constructor
pub const PAGE_MEMORY_SIZE: u64 = 64 * 1024;
pub const MAX_MEMORY_SIZE: u64 = (MAX_PAGES as u64) * PAGE_MEMORY_SIZE;

/** Gas cost for custom functions */
pub const LOAD_COST: u64 = 20_000_000;
pub const STORE_COST: u64 = 140_000_000;

pub const CALL_COST: u64 = 343_000_000;
pub const DEPLOY_COST: u64 = 2_500_000_000;
pub const ENCODE_ADDRESS_COST: u64 = 1_000_000;

pub const SHA256_COST: u64 = 50_000;
pub const RIMD160_COST: u64 = 50_000;

pub const INPUTS_COST: u64 = 5_000_000;
pub const OUTPUTS_COST: u64 = 5_000_000;

pub const EMIT_COST: u64 = 1_000_000;
