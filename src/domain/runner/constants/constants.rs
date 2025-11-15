pub const MAX_PAGES: u32 = 1024; // 1 page = 64 KiB, 32 MiB.
pub const STACK_SIZE: usize = 1024 * 1024; // 1 MiB
pub const PAGE_MEMORY_SIZE: u64 = 64 * 1024;
pub const MAX_MEMORY_SIZE: u64 = (MAX_PAGES as u64) * PAGE_MEMORY_SIZE;
pub const MAX_GAS_WASM_INIT: u64 = 20_000_000;
pub const MAX_MEMORY_COPY_SIZE: u32 = 1024 * 1024; // 1 MiB
pub const COLD_ADDRESS_ACCESS_GAS_COST: u64 = 26_000_000;
pub const WARM_ADDRESS_ACCESS_GAS_COST: u64 = 1_000_000;
pub const COLD_STORAGE_GAS_COST: u64 = 21_000_000;
pub const WARM_STORAGE_GAS_COST: u64 = 1_000_000;
pub const NEW_STORAGE_SLOT_GAS_COST: u64 = 200_000_000;
pub const UPDATED_STORAGE_SLOT_GAS_COST: u64 = 29_000_000;
pub const LOAD_MLDSA_PUBLIC_KEY_GAS_COST: u64 = 100_000_000;

#[cfg(feature = "contract-threading")]
pub const MAX_GAS_WASM_INIT_ATOMIC: u64 = 1_000_000;
#[cfg(feature = "contract-threading")]
pub const MAX_NS: u64 = 1_000_000;
#[cfg(feature = "contract-threading")]
pub const NS_PER_HASH: u64 = 100_000_000; // 100ms
#[cfg(feature = "contract-threading")]
pub const MAX_DIFF: u64 = 5_000; // hard ceiling
#[cfg(feature = "contract-threading")]
pub const MAX_THREADS: i32 = 8; // max number of threads
