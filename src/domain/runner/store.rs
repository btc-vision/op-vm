use std::collections::BTreeMap;

pub const STORAGE_KEY_SIZE: usize = 32;
pub const STORAGE_VALUE_SIZE: usize = 32;

pub type StorageKey = [u8; STORAGE_KEY_SIZE];
pub type StorageValue = [u8; STORAGE_VALUE_SIZE];

pub const STORAGE_VALUE_ZERO: [u8; STORAGE_VALUE_SIZE] = [0; STORAGE_VALUE_SIZE];

pub struct TransientStorage(BTreeMap<StorageKey, StorageValue>);

impl TransientStorage {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn get(&self, key: &StorageKey) -> StorageValue {
        self.0
            .get(key)
            .cloned()
            .unwrap_or_else(|| STORAGE_VALUE_ZERO)
    }

    pub fn set(&mut self, key: StorageKey, value: StorageValue) {
        self.0.insert(key, value);
    }
}
