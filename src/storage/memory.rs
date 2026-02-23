use std::cell::RefCell;
use std::collections::HashMap;

pub struct MemoryStorage {
    storage: RefCell<HashMap<String, String>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            storage: RefCell::new(HashMap::new()),
        }
    }

    pub fn put(&self, key: impl Into<String>, value: impl Into<String>) -> Option<String> {
        self.storage.borrow_mut().insert(key.into(), value.into())
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<String> {
        self.storage.borrow().get(key.as_ref()).cloned()
    }
}
