use std::collections::HashMap;
use wasmer::RuntimeError;

pub const LOAD_COLD_GAS_COST: u64 = 21_000_000;
pub const LOAD_WARM_GAS_COST: u64 = 1_000_000;

pub const STORE_BASE_GAS_COST: u64 = 1_000_000;
pub const STORE_NEW_GAS_COST: u64 = 200_000_000;
pub const STORE_UPDATE_GAS_COST: u64 = 29_000_000;
pub const STORE_REFUND_GAS_COST: u64 = 48_000_000;

pub const STORAGE_POINTER_SIZE: usize = 32;
pub const STORAGE_VALUE_SIZE: usize = 32;
pub type StoragePointer = [u8; STORAGE_POINTER_SIZE];
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
    values: HashMap<[u8; STORAGE_POINTER_SIZE], CacheValue>,
    reads: usize,
    writes: usize,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            reads: 0,
            writes: 0,
        }
    }

    pub fn get<GetFun>(
        &mut self,
        pointer: &[u8; STORAGE_POINTER_SIZE],
        get_fn: GetFun,
    ) -> Result<CacheResponse, RuntimeError>
    where
        GetFun: Fn(StoragePointer) -> Result<(StorageValue, bool), RuntimeError>,
    {
        self.reads += 1;

        // If object exists in the cache
        if let Some(value) = self.values.get(pointer) {
            Ok(CacheResponse::new(value.current, LOAD_WARM_GAS_COST, 0))
        }
        // first access of pointer
        else {
            let original_value = get_fn(*pointer)?;
            self.values
                .insert(*pointer, CacheValue::new(original_value.0));

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
        pointer: [u8; STORAGE_POINTER_SIZE],
        value: [u8; STORAGE_VALUE_SIZE],
        get_value: GetFun,
        set_value: SetFun,
    ) -> Result<CacheResponse, RuntimeError>
    where
        GetFun: Fn(StoragePointer) -> Result<(StorageValue, bool), RuntimeError>,
        SetFun: Fn(StoragePointer, StorageValue) -> Result<(), RuntimeError>,
    {
        // increase number of writes
        self.writes += 1;

        let mut gas_cost = 0;
        let mut gas_refund: i64 = 0;

        let mut cache_value = if let Some(value) = self.values.get(&pointer) {
            // Warm access
            value.clone()
        } else {
            // Cold access
            let data = get_value(pointer)?;

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
            set_value(pointer, value)?;
            self.values.insert(pointer, cache_value);
        }

        Ok(CacheResponse::new(value, gas_cost, gas_refund))
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.values.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::{Cache, CacheValue, StoragePointer, StorageValue};
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    const POINTER: StoragePointer = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];

    fn create_store(
        pointer: Option<StoragePointer>,
        original_value: Option<StorageValue>,
        current_value: Option<StorageValue>,
    ) -> (Cache, Arc<Mutex<HashMap<StoragePointer, StorageValue>>>) {
        let pointer = pointer.unwrap_or([1; super::STORAGE_POINTER_SIZE]);
        let store: Arc<Mutex<HashMap<StoragePointer, StorageValue>>> = if original_value.is_some() {
            Arc::new(Mutex::new(HashMap::from_iter([(
                pointer,
                original_value.unwrap(),
            )])))
        } else {
            Arc::new(Mutex::new(HashMap::new()))
        };

        let cache = if current_value.is_some() {
            Cache {
                reads: 0,
                writes: 0,
                values: HashMap::from_iter([(
                    pointer,
                    CacheValue {
                        original: original_value.unwrap_or(super::STORAGE_VALUE_ZERO),
                        current: current_value.unwrap(),
                    },
                )]),
            }
        } else {
            Cache::new()
        };

        (cache, store)
    }

    #[test]
    pub fn test_load_value() {
        let value = [10; 32];

        let (mut cache, store) = create_store(Some(POINTER), Some(value), None);
        let get_fn = |pointer: StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&pointer)
                .unwrap_or(&super::STORAGE_VALUE_ZERO)
                .clone())
        };

        // Cold value
        let result = cache.get(&POINTER, get_fn).unwrap();
        assert_eq!(result.gas_cost, 21_000_000);
        assert_eq!(result.value, value);

        // Warm value
        let result = cache.get(&POINTER, get_fn).unwrap();
        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.value, value);
    }

    #[test]
    pub fn test_store_new_cold_value() {
        let (mut cache, store) = create_store(Some(POINTER), None, None);
        let get_fn = |pointer: StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&pointer)
                .unwrap_or(&super::STORAGE_VALUE_ZERO)
                .clone())
        };
        let set_fn = |pointer: StoragePointer, value: StorageValue| {
            store.lock().unwrap().insert(pointer, value);
            Ok(())
        };

        let result = cache.set(POINTER, [1; 32], get_fn, set_fn).unwrap();
        assert_eq!(store.lock().unwrap().get(&POINTER).unwrap(), &[1; 32]);

        assert_eq!(result.gas_cost, 221_000_000);
        assert_eq!(result.gas_refund, 0);
    }

    #[test]
    pub fn test_store_warm_not_changed_value() {
        let (mut cache, store) = create_store(Some(POINTER), None, None);
        let get_fn = |pointer: StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&pointer)
                .unwrap_or(&super::STORAGE_VALUE_ZERO)
                .clone())
        };
        let set_fn = |pointer: StoragePointer, value: StorageValue| {
            store.lock().unwrap().insert(pointer, value);
            Ok(())
        };

        // Warm - not changed
        let _result = cache.set(POINTER, [1; 32], get_fn, set_fn).unwrap();
        let result = cache.set(POINTER, [1; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, 0);
    }

    #[test]
    pub fn test_store_cold_changed() {
        let (mut cache, store) = create_store(Some(POINTER), None, None);
        let get_fn = |pointer: StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&pointer)
                .unwrap_or(&super::STORAGE_VALUE_ZERO)
                .clone())
        };
        let set_fn = |pointer: StoragePointer, value: StorageValue| {
            store.lock().unwrap().insert(pointer, value);
            Ok(())
        };

        // cold - changed
        let _result = cache.set(POINTER, [1; 32], get_fn, set_fn).unwrap();
        cache.reset();
        let result = cache.set(POINTER, [2; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 50_000_000);
        assert_eq!(result.gas_refund, 0);
    }

    #[test]
    pub fn test_store_warm_changed_after_change() {
        let (mut cache, store) = create_store(Some(POINTER), None, None);
        let get_fn = |pointer: StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&pointer)
                .unwrap_or(&super::STORAGE_VALUE_ZERO)
                .clone())
        };
        let set_fn = |pointer: StoragePointer, value: StorageValue| {
            store.lock().unwrap().insert(pointer, value);
            Ok(())
        };

        // warm changed - after change
        let _result = cache.set(POINTER, [2; 32], get_fn, set_fn).unwrap();
        let result = cache.set(POINTER, [3; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, 0);
    }

    #[test]
    pub fn test_store_warm_changed() {
        let (mut cache, store) = create_store(Some(POINTER), None, None);
        let get_fn = |pointer: StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&pointer)
                .unwrap_or(&super::STORAGE_VALUE_ZERO)
                .clone())
        };
        let set_fn = |pointer: StoragePointer, value: StorageValue| {
            store.lock().unwrap().insert(pointer, value);
            Ok(())
        };

        let _result = cache.set(POINTER, [3; 32], get_fn, set_fn).unwrap();
        cache.reset();
        let result = cache.get(&POINTER, get_fn).unwrap();
        assert_eq!(result.value, [3; 32]);

        let result = cache.set(POINTER, [4; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 29_000_000);
        assert_eq!(result.gas_refund, 0);
    }

    #[test]
    pub fn test_store_cold_reset() {
        let (mut cache, store) = create_store(Some(POINTER), None, None);
        let get_fn = |pointer: StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&pointer)
                .unwrap_or(&super::STORAGE_VALUE_ZERO)
                .clone())
        };
        let set_fn = |pointer: StoragePointer, value: StorageValue| {
            store.lock().unwrap().insert(pointer, value);
            Ok(())
        };

        // Cold reset
        let _result = cache.set(POINTER, [4; 32], get_fn, set_fn).unwrap();
        cache.reset();
        let result = cache.set(POINTER, [0; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 50_000_000);
        assert_eq!(result.gas_refund, 48_000_000);
    }

    #[test]
    pub fn test_store_warm_reset() {
        let (mut cache, store) = create_store(Some(POINTER), None, None);
        let get_fn = |pointer: StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&pointer)
                .unwrap_or(&super::STORAGE_VALUE_ZERO)
                .clone())
        };
        let set_fn = |pointer: StoragePointer, value: StorageValue| {
            store.lock().unwrap().insert(pointer, value);
            Ok(())
        };

        // warm reset
        cache.set(POINTER, [1; 32], get_fn, set_fn).unwrap();
        cache.reset();

        let result = cache.get(&POINTER, get_fn).unwrap();
        assert_eq!(result.value, [1; 32]);

        let result = cache.set(POINTER, [0; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 29_000_000);
        assert_eq!(result.gas_refund, 48_000_000);
    }

    #[test]
    pub fn test_store_default_change_default() {
        let (mut cache, store) = create_store(Some(POINTER), None, None);
        let get_fn = |pointer: StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&pointer)
                .unwrap_or(&super::STORAGE_VALUE_ZERO)
                .clone())
        };
        let set_fn = |pointer: StoragePointer, value: StorageValue| {
            store.lock().unwrap().insert(pointer, value);
            Ok(())
        };
        // Zero to default, change, default
        cache.reset();
        cache.set(POINTER, [1; 32], get_fn, set_fn).unwrap();
        let result = cache.set(POINTER, [0; 32], get_fn, set_fn).unwrap();

        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, 199_000_000);
    }

    #[test]
    pub fn test_store_zero_same_value() {
        let (mut cache, store) = create_store(Some(POINTER), None, None);
        let get_fn = |pointer: StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&pointer)
                .unwrap_or(&super::STORAGE_VALUE_ZERO)
                .clone())
        };
        let set_fn = |pointer: StoragePointer, value: StorageValue| {
            store.lock().unwrap().insert(pointer, value);
            Ok(())
        };

        // Value zero same value
        cache.set(POINTER, [1; 32], get_fn, set_fn).unwrap();
        cache.reset();
        cache.set(POINTER, [0; 32], get_fn, set_fn).unwrap();

        let result = cache.set(POINTER, [1; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, -20_000_000);
    }

    #[test]
    pub fn test_store_zero_different_value() {
        let (mut cache, store) = create_store(Some(POINTER), None, None);
        let get_fn = |pointer: StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&pointer)
                .unwrap_or(&super::STORAGE_VALUE_ZERO)
                .clone())
        };
        let set_fn = |pointer: StoragePointer, value: StorageValue| {
            store.lock().unwrap().insert(pointer, value);
            Ok(())
        };

        // Value zero different value
        cache.set(POINTER, [1; 32], get_fn, set_fn).unwrap();
        cache.reset();
        cache.set(POINTER, [0; 32], get_fn, set_fn).unwrap();
        let result = cache.set(POINTER, [2; 32], get_fn, set_fn).unwrap();

        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, -48_000_000);
    }
}
