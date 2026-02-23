mod memory;
mod redis;

use memory::*;
use redis::*;

pub enum Storage {
    Memory(MemoryStorage),
    Redis(RedisStorage),
}

impl Storage {
    pub fn new_memory() -> Self {
        Self::Memory(MemoryStorage::new())
    }

    pub fn new_redis(connection_manager: ::redis::aio::ConnectionManager) -> Self {
        Self::Redis(RedisStorage::new(connection_manager))
    }

    pub fn put(&self, key: impl Into<String>, value: impl Into<String>) -> Option<String> {
        match self {
            Self::Memory(storage) => storage.put(key, value),
            Self::Redis(storage) => storage.put(key, value),
        }
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<String> {
        match self {
            Self::Memory(storage) => storage.get(key),
            Self::Redis(storage) => storage.get(key),
        }
    }
}
