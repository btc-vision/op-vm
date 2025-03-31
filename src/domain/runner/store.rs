use std::collections::HashMap;
use wasmer::RuntimeError;

pub const LOAD_COLD_GAS_COST: u64 = 21_000_000;
pub const LOAD_WARM_GAS_COST: u64 = 1_000_000;

pub const STORE_BASE_GAS_COST: u64 = 1_000_000;
pub const STORE_NEW_GAS_COST: u64 = 200_000_000;
pub const STORE_UPDATE_GAS_COST: u64 = 29_000_000;
pub const STORE_REFUND_GAS_COST: u64 = 48_000_000;

pub const STORAGE_KEY_SIZE: usize = 32;
pub const STORAGE_VALUE_SIZE: usize = 32;
pub type StorageKey = [u8; STORAGE_KEY_SIZE];
pub type StorageValue = [u8; STORAGE_VALUE_SIZE];
pub const STORAGE_VALUE_ZERO: [u8; STORAGE_VALUE_SIZE] = [0; STORAGE_VALUE_SIZE];

pub struct CacheResponse {
    pub value: StorageValue,
    pub gas_cost: u64,
    pub gas_refund: i64,
}

#[derive(Debug, Clone)]
pub struct CacheValue {
    original: StorageValue,
    current: StorageValue,
}

impl CacheValue {
    // New value filled from existing store
    pub fn new(value: StorageValue) -> Self {
        Self {
            original: value,
            current: value,
        }
    }
}

impl CacheResponse {
    pub fn new(value: [u8; STORAGE_VALUE_SIZE], gas_cost: u64, gas_refund: i64) -> Self {
        Self {
            value,
            gas_cost,
            gas_refund,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cache {
    values: HashMap<[u8; STORAGE_KEY_SIZE], CacheValue>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get<GetFun>(
        &mut self,
        key: &[u8; STORAGE_KEY_SIZE],
        get_fn: GetFun,
    ) -> Result<CacheResponse, RuntimeError>
    where
        GetFun: Fn(StorageKey) -> Result<(StorageValue, bool), RuntimeError>,
    {
        // If object exists in the cache
        if let Some(value) = self.values.get(key) {
            Ok(CacheResponse::new(value.current, LOAD_WARM_GAS_COST, 0))
        }
        // First access of key
        else {
            let original_value = get_fn(*key)?;
            self.values.insert(*key, CacheValue::new(original_value.0));

            let is_cold = original_value.1;
            if is_cold {
                Ok(CacheResponse::new(original_value.0, LOAD_COLD_GAS_COST, 0))
            } else {
                Ok(CacheResponse::new(original_value.0, LOAD_WARM_GAS_COST, 0))
            }
        }
    }

    pub fn set<GetFun, SetFun>(
        &mut self,
        key: [u8; STORAGE_KEY_SIZE],
        value: [u8; STORAGE_VALUE_SIZE],
        get_value: GetFun,
        set_value: SetFun,
    ) -> Result<CacheResponse, RuntimeError>
    where
        GetFun: Fn(StorageKey) -> Result<(StorageValue, bool), RuntimeError>,
        SetFun: Fn(StorageKey, StorageValue) -> Result<(), RuntimeError>,
    {
        let mut gas_cost = 0;
        let mut gas_refund: i64 = 0;

        let mut cache_value = if let Some(value) = self.values.get(&key) {
            // Warm access
            value.clone()
        } else {
            // Cold access
            let data = get_value(key)?;

            if data.1 {
                gas_cost += LOAD_COLD_GAS_COST;
            } else {
                gas_cost += LOAD_WARM_GAS_COST;
            }

            CacheValue::new(data.0)
        };

        if value == cache_value.current {
            // No changes
            gas_cost += STORE_BASE_GAS_COST;
        } else {
            // Clean slot (not yet updated in current execution context)
            if cache_value.original == cache_value.current {
                // (slot started zero, currently still zero, now being changed to nonzero)
                if cache_value.original == STORAGE_VALUE_ZERO {
                    gas_cost += STORE_NEW_GAS_COST;
                }
                // (slot started nonzero, currently still same nonzero value, now being changed):
                else {
                    gas_cost += STORE_UPDATE_GAS_COST;
                }
            } else {
                gas_cost += STORE_BASE_GAS_COST
            };

            if cache_value.current == cache_value.original {
                if cache_value.original != STORAGE_VALUE_ZERO && value == STORAGE_VALUE_ZERO {
                    gas_refund = STORE_REFUND_GAS_COST as i64;
                }
            } else {
                if cache_value.original != STORAGE_VALUE_ZERO {
                    if cache_value.current == STORAGE_VALUE_ZERO {
                        if value != cache_value.original {
                            gas_refund -= STORE_REFUND_GAS_COST as i64;
                        } else {
                            gas_refund -= (LOAD_COLD_GAS_COST as i64) - (LOAD_WARM_GAS_COST as i64);
                        }
                    } else if value == STORAGE_VALUE_ZERO {
                        gas_refund = STORE_REFUND_GAS_COST as i64;
                    }
                } else if cache_value.original == value {
                    if cache_value.original == STORAGE_VALUE_ZERO {
                        gas_refund = (STORE_NEW_GAS_COST as i64) - (STORE_BASE_GAS_COST as i64);
                    } else {
                        gas_refund = (STORE_REFUND_GAS_COST as i64) - (LOAD_COLD_GAS_COST as i64)
                            + (LOAD_WARM_GAS_COST as i64);
                    }
                }
            }

            // Set value to store
            cache_value.current = value;
            set_value(key, value)?;
            self.values.insert(key, cache_value);
        }

        Ok(CacheResponse::new(value, gas_cost, gas_refund))
    }
}

#[cfg(test)]
mod tests {
    use super::{Cache, StorageKey, StorageValue};
    use crate::domain::runner::STORAGE_VALUE_ZERO;
    use std::cell::RefCell;
    use wasmer::RuntimeError;

    const A_STORAGE_KEY: StorageKey = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];

    macro_rules! create_getter_setter_mocks {
        ($getter_value:expr) => {{
            let get_fn = |_: StorageKey| Ok(($getter_value, true));
            let set_fn = |_: StorageKey, _: StorageValue| Ok::<(), RuntimeError>(());
            (get_fn, set_fn)
        }};
    }

    #[test]
    pub fn load_cold_value() {
        const STORE_VALUE: StorageValue = [10; 32];
        let mut cache = Cache::new();
        let (get_fn, _) = create_getter_setter_mocks!(STORE_VALUE);

        let result = cache.get(&A_STORAGE_KEY, get_fn).unwrap();

        assert_eq!(result.gas_cost, 21_000_000);
        assert_eq!(result.value, STORE_VALUE);
    }

    #[test]
    pub fn load_warm_value() {
        const STORE_VALUE: StorageValue = [10; 32];
        let mut cache = Cache::new();
        let (get_fn, _) = create_getter_setter_mocks!(STORE_VALUE);

        cache.get(&A_STORAGE_KEY, get_fn).unwrap();
        let result = cache.get(&A_STORAGE_KEY, get_fn).unwrap();

        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.value, STORE_VALUE);
    }

    #[test]
    pub fn given_a_cold_slot_with_empty_original_value_store_new_value() {
        let mut cache = Cache::new();
        let value_in_store: RefCell<StorageValue> = RefCell::from(STORAGE_VALUE_ZERO);
        let get_fn = |_: StorageKey| Ok((STORAGE_VALUE_ZERO, true));
        let set_fn = |_: StorageKey, value: StorageValue| {
            *value_in_store.borrow_mut() = value;
            Ok(())
        };

        let result = cache.set(A_STORAGE_KEY, [1; 32], get_fn, set_fn).unwrap();

        assert_eq!(*value_in_store.borrow(), [1; 32]);
        assert_eq!(result.gas_cost, 221_000_000);
        assert_eq!(result.gas_refund, 0);
    }

    #[test]
    pub fn given_a_warm_slot_with_empty_original_value_and_a_current_value_store_same_value() {
        let mut cache = Cache::new();
        let (get_fn, set_fn) = create_getter_setter_mocks!(STORAGE_VALUE_ZERO);

        cache.set(A_STORAGE_KEY, [1; 32], get_fn, set_fn).unwrap();
        let result = cache.set(A_STORAGE_KEY, [1; 32], get_fn, set_fn).unwrap();

        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, 0);
    }

    #[test]
    pub fn given_a_cold_slot_with_non_empty_original_value_store_new_value() {
        const STORE_VALUE: [u8; 32] = [3; 32];
        let mut cache = Cache::new();
        let (get_fn, set_fn) = create_getter_setter_mocks!(STORE_VALUE);

        let result = cache.set(A_STORAGE_KEY, [2; 32], get_fn, set_fn).unwrap();

        assert_eq!(result.gas_cost, 50_000_000);
        assert_eq!(result.gas_refund, 0);
    }

    #[test]
    pub fn given_a_warm_slot_with_empty_original_value_and_a_current_value_store_new_value() {
        let mut cache = Cache::new();
        let (get_fn, set_fn) = create_getter_setter_mocks!(STORAGE_VALUE_ZERO);

        // warm changed - after change
        cache.set(A_STORAGE_KEY, [2; 32], get_fn, set_fn).unwrap();
        let result = cache.set(A_STORAGE_KEY, [3; 32], get_fn, set_fn).unwrap();

        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, 0);
    }

    #[test]
    pub fn given_a_warm_slot_with_an_original_value_store_new_value() {
        const STORE_VALUE: [u8; 32] = [3; 32];
        let mut cache = Cache::new();
        let (get_fn, set_fn) = create_getter_setter_mocks!(STORE_VALUE);

        cache.get(&A_STORAGE_KEY, get_fn).unwrap();
        let result = cache.set(A_STORAGE_KEY, [4; 32], get_fn, set_fn).unwrap();

        assert_eq!(result.gas_cost, 29_000_000);
        assert_eq!(result.gas_refund, 0);
    }

    #[test]
    pub fn given_a_cold_slot_with_an_original_value_store_empty_value() {
        const STORE_VALUE: [u8; 32] = [4; 32];
        let mut cache = Cache::new();
        let (get_fn, set_fn) = create_getter_setter_mocks!(STORE_VALUE);

        let result = cache
            .set(A_STORAGE_KEY, STORAGE_VALUE_ZERO, get_fn, set_fn)
            .unwrap();

        assert_eq!(result.gas_cost, 50_000_000);
        assert_eq!(result.gas_refund, 48_000_000);
    }

    #[test]
    pub fn given_a_warm_slot_with_an_original_value_store_empty_value() {
        const STORE_VALUE: [u8; 32] = [1; 32];
        let mut cache = Cache::new();
        let (get_fn, set_fn) = create_getter_setter_mocks!(STORE_VALUE);

        cache.get(&A_STORAGE_KEY, get_fn).unwrap();
        let result = cache
            .set(A_STORAGE_KEY, STORAGE_VALUE_ZERO, get_fn, set_fn)
            .unwrap();

        assert_eq!(result.gas_cost, 29_000_000);
        assert_eq!(result.gas_refund, 48_000_000);
    }

    #[test]
    pub fn given_a_warm_slot_with_empty_original_value_and_a_current_value_store_empty_value() {
        let mut cache = Cache::new();
        let (get_fn, set_fn) = create_getter_setter_mocks!(STORAGE_VALUE_ZERO);

        cache.set(A_STORAGE_KEY, [1; 32], get_fn, set_fn).unwrap();
        let result = cache
            .set(A_STORAGE_KEY, STORAGE_VALUE_ZERO, get_fn, set_fn)
            .unwrap();

        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, 199_000_000);
    }

    #[test]
    pub fn given_a_warm_slot_with_an_original_value_and_empty_current_value_store_original_value() {
        const STORE_VALUE: [u8; 32] = [1; 32];
        let mut cache = Cache::new();
        let (get_fn, set_fn) = create_getter_setter_mocks!(STORE_VALUE);

        cache
            .set(A_STORAGE_KEY, STORAGE_VALUE_ZERO, get_fn, set_fn)
            .unwrap();
        let result = cache.set(A_STORAGE_KEY, [1; 32], get_fn, set_fn).unwrap();

        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, -20_000_000);
    }

    #[test]
    pub fn given_a_warm_slot_with_an_original_value_and_empty_current_value_store_new_value() {
        const STORE_VALUE: [u8; 32] = [1; 32];
        let mut cache = Cache::new();
        let (get_fn, set_fn) = create_getter_setter_mocks!(STORE_VALUE);

        cache
            .set(A_STORAGE_KEY, STORAGE_VALUE_ZERO, get_fn, set_fn)
            .unwrap();
        let result = cache.set(A_STORAGE_KEY, [2; 32], get_fn, set_fn).unwrap();

        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, -48_000_000);
    }
}
