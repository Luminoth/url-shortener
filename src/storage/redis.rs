pub struct RedisStorage {
    connection_manager: redis::aio::ConnectionManager,
}

impl RedisStorage {
    pub fn new(connection_manager: redis::aio::ConnectionManager) -> Self {
        Self { connection_manager }
    }

    pub fn put(&self, key: impl Into<String>, value: impl Into<String>) -> Option<String> {
        todo!()
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<String> {
        todo!()
    }
}
