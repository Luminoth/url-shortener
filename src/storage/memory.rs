use moka::future::Cache;

#[derive(Debug, Clone)]
pub struct MemoryStorage {
    storage: Cache<String, String>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            storage: Cache::builder().build(),
        }
    }

    pub async fn put(&self, key: impl Into<String>, value: impl Into<String>) {
        self.storage.insert(key.into(), value.into()).await
    }

    pub async fn get(&self, key: impl AsRef<str>) -> Option<String> {
        self.storage.get(key.as_ref()).await
    }
}
