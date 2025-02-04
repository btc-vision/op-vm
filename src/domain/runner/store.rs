use super::constants::{LOAD_COLD, LOAD_WARM, STORE_BASE, STORE_NEW, STORE_REFUND, STORE_UPDATE};
use wasmer::RuntimeError;

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
    values: std::collections::HashMap<[u8; STORAGE_POINTER_SIZE], CacheValue>,
    reads: usize,
    writes: usize,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
            reads: 0,
            writes: 0,
        }
    }

    #[allow(dead_code)]
    pub fn exists(&self, pointer: &[u8; STORAGE_POINTER_SIZE]) -> bool {
        self.values.contains_key(pointer)
    }

    pub fn get<GetFun>(
        &mut self,
        pointer: &[u8; STORAGE_POINTER_SIZE],
        get_fn: GetFun,
    ) -> Result<CacheResponse, RuntimeError>
    where
        GetFun: Fn(StoragePointer) -> Result<StorageValue, RuntimeError>,
    {
        self.reads += 1;

        // If object exists in the cache
        if let Some(value) = self.values.get(pointer) {
            Ok(CacheResponse::new(value.current, LOAD_WARM, 0))
        }
        // first acces of pointer
        else {
            let original_value = get_fn(*pointer)?;
            self.values
                .insert(*pointer, CacheValue::new(original_value));

            Ok(CacheResponse::new(original_value, LOAD_COLD, 0))
        }
    }

    pub fn set<GetFun, SetFun>(
        &mut self,
        key: [u8; STORAGE_POINTER_SIZE],
        value: [u8; STORAGE_VALUE_SIZE],
        get_value: GetFun,
        set_value: SetFun,
    ) -> Result<CacheResponse, RuntimeError>
    where
        GetFun: Fn(StoragePointer) -> Result<StorageValue, RuntimeError>,
        SetFun: Fn(StoragePointer, StorageValue) -> Result<(), RuntimeError>,
    {
        // increase number of writes
        self.writes += 1;

        let mut gas_cost = 0;
        let mut gas_refund: i64 = 0;

        let mut cache_value = if let Some(value) = self.values.get(&key) {
            // Warm access
            value.clone()
        } else {
            // Cold access
            gas_cost += LOAD_COLD;
            CacheValue::new(get_value(key)?)
        };

        if value == cache_value.current {
            // No changes
            gas_cost += STORE_BASE;
        } else {
            // Clean slot (not yet updated in current execution context)
            if cache_value.original == cache_value.current {
                // (slot started zero, currently still zero, now being changed to nonzero)
                if cache_value.original == STORAGE_VALUE_ZERO {
                    gas_cost += STORE_NEW;
                }
                // (slot started nonzero, currently still same nonzero value, now being changed):
                else {
                    gas_cost += STORE_UPDATE;
                }
            } else {
                gas_cost += STORE_BASE
            };

            if cache_value.current == cache_value.original {
                if cache_value.original != STORAGE_VALUE_ZERO && value == STORAGE_VALUE_ZERO {
                    gas_refund = STORE_REFUND as i64;
                }
            } else {
                if cache_value.original != STORAGE_VALUE_ZERO {
                    if cache_value.current == STORAGE_VALUE_ZERO {
                        // TODO: Add to gas???
                        if value != cache_value.original {
                            gas_refund -= STORE_REFUND as i64;
                        } else {
                            gas_refund -= (LOAD_COLD as i64) - (LOAD_WARM as i64);
                        }
                    } else if value == STORAGE_VALUE_ZERO {
                        gas_refund = STORE_REFUND as i64;
                    }
                } else if cache_value.original == value {
                    if cache_value.original == STORAGE_VALUE_ZERO {
                        gas_refund = (STORE_NEW as i64) - (STORE_BASE as i64);
                    } else {
                        gas_refund =
                            (STORE_REFUND as i64) - (LOAD_COLD as i64) + (LOAD_WARM as i64);
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

    const POINTER: super::StoragePointer = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];

    fn create_store(
        pointer: Option<StoragePointer>,
        original_value: Option<StorageValue>,
        current_value: Option<StorageValue>,
    ) -> (
        Cache,
        Arc<Mutex<HashMap<super::StoragePointer, super::StorageValue>>>,
    ) {
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
    pub fn test_cold_write_zero() {
        let value = [0; 32];

        let (mut cache, store) = create_store(Some(POINTER), None, None);
        let get_fn = |key: super::StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&key)
                .unwrap_or(&super::STORAGE_VALUE_ZERO)
                .clone())
        };
        let set_fn = |key: super::StoragePointer, value: super::StorageValue| {
            store.lock().unwrap().insert(key, value);
            Ok(())
        };

        cache.get(&POINTER, get_fn).unwrap();

        let result = cache.set(POINTER, value, get_fn, set_fn).unwrap();

        assert_eq!(result.gas_cost, 1_000_000);
    }

    #[test]
    pub fn test_get_value() {
        let value = [10; 32];

        let (mut cache, store) = create_store(Some(POINTER), Some(value), None);
        let get_fn = |key: super::StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&key)
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
    pub fn test_set_value() {
        let key = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ];

        let (mut cache, store) = create_store(Some(key), None, None);
        let get_fn = |key: super::StoragePointer| {
            Ok(store
                .lock()
                .unwrap()
                .get(&key)
                .unwrap_or(&super::STORAGE_VALUE_ZERO)
                .clone())
        };
        let set_fn = |key: super::StoragePointer, value: super::StorageValue| {
            store.lock().unwrap().insert(key, value);
            Ok(())
        };

        // New value / cold
        let result = cache.set(key, [1; 32], get_fn, set_fn).unwrap();
        assert_eq!(store.lock().unwrap().get(&key).unwrap(), &[1; 32]);

        assert_eq!(result.gas_cost, 221_000_000);
        assert_eq!(result.gas_refund, 0);

        // Warm - not changed
        let result = cache.set(key, [1; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, 0);

        cache.reset();

        // cold - changed
        let result = cache.set(key, [2; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 50_000_000);
        assert_eq!(result.gas_refund, 0);

        // warm changed - after change
        let result = cache.set(key, [3; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, 0);

        // Warm change
        cache.reset();
        let result = cache.get(&key, get_fn).unwrap();
        assert_eq!(result.value, [3; 32]);

        let result = cache.set(key, [4; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 29_000_000);
        assert_eq!(result.gas_refund, 0);

        // Cold reset
        cache.reset();
        let result = cache.set(key, [0; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 50_000_000);
        assert_eq!(result.gas_refund, 48_000_000);

        // warm reset
        cache.set(key, [1; 32], get_fn, set_fn).unwrap();
        cache.reset();

        let result = cache.get(&key, get_fn).unwrap();
        assert_eq!(result.value, [1; 32]);

        let result = cache.set(key, [0; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 29_000_000);
        assert_eq!(result.gas_refund, 48_000_000);

        // Zero to default, change, default
        cache.reset();
        cache.set(key, [1; 32], get_fn, set_fn).unwrap();
        let result = cache.set(key, [0; 32], get_fn, set_fn).unwrap();

        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, 199_000_000);

        // Value zero same value
        cache.set(key, [1; 32], get_fn, set_fn).unwrap();
        cache.reset();
        cache.set(key, [0; 32], get_fn, set_fn).unwrap();

        let result = cache.set(key, [1; 32], get_fn, set_fn).unwrap();
        assert_eq!(result.gas_cost, 1_000_000);

        // TODO: This values is not in formula: https://www.evm.codes/?fork=cancun#55
        // but dialog calculation give this result
        assert_eq!(result.gas_refund, -20_000_000);

        // Value zero different value
        cache.set(key, [1; 32], get_fn, set_fn).unwrap();
        cache.reset();
        cache.set(key, [0; 32], get_fn, set_fn).unwrap();
        let result = cache.set(key, [2; 32], get_fn, set_fn).unwrap();

        assert_eq!(result.gas_cost, 1_000_000);
        assert_eq!(result.gas_refund, -48_000_000);
    }
}
