mod memory;
mod redis;

use memory::*;
use redis::*;

#[derive(Debug, Clone)]
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

    pub async fn put(&self, id: impl Into<String>, url: impl Into<String>) -> crate::Result<()> {
        match self {
            Self::Memory(storage) => {
                storage.put(id, url).await;
                Ok(())
            }
            Self::Redis(storage) => storage.put(id, url).await,
        }
    }

    pub async fn get(&self, id: impl AsRef<str>) -> crate::Result<Option<String>> {
        match self {
            Self::Memory(storage) => Ok(storage.get(id).await),
            Self::Redis(storage) => storage.get(id).await,
        }
    }
}
