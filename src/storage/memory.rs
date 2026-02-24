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

    pub async fn put(&self, id: impl Into<String>, url: impl Into<String>) {
        self.storage.insert(id.into(), url.into()).await
    }

    pub async fn get(&self, id: impl AsRef<str>) -> Option<String> {
        self.storage.get(id.as_ref()).await
    }
}
