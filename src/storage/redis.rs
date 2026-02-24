use redis::AsyncCommands;

#[derive(Debug, Clone)]
pub struct RedisStorage {
    connection_manager: redis::aio::ConnectionManager,
}

impl RedisStorage {
    pub fn new(connection_manager: redis::aio::ConnectionManager) -> Self {
        Self { connection_manager }
    }

    pub async fn put(&self, id: impl Into<String>, url: impl Into<String>) -> crate::Result<()> {
        let id = id.into();
        let url = url.into();

        let mut conn = self.connection_manager.clone();

        let _result: i32 = conn.hset("short_urls", id, url).await?;
        // TODO: should we verify the result?

        Ok(())
    }

    pub async fn get(&self, id: impl AsRef<str>) -> crate::Result<Option<String>> {
        let mut conn = self.connection_manager.clone();

        let result: Option<String> = conn.hget("short_urls", id.as_ref()).await?;
        Ok(result)
    }
}
