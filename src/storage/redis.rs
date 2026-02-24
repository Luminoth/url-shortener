#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct RedisStorage {
    connection_manager: redis::aio::ConnectionManager,
}

impl RedisStorage {
    pub fn new(connection_manager: redis::aio::ConnectionManager) -> Self {
        Self { connection_manager }
    }

    pub async fn put(&self, _key: impl Into<String>, _value: impl Into<String>) {
        todo!()
    }

    pub async fn get(&self, _key: impl AsRef<str>) -> Option<String> {
        todo!()
    }
}
