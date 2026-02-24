#![deny(warnings)]

mod error;
mod storage;

use tracing::debug;
use uuid::Uuid;

pub use error::*;
use storage::*;

#[derive(Debug, Clone)]
pub struct UrlShortener {
    storage: Storage,
}

impl UrlShortener {
    pub fn new_memory() -> Self {
        Self {
            storage: Storage::new_memory(),
        }
    }

    pub fn new_redis(conection_manager: ::redis::aio::ConnectionManager) -> Self {
        Self {
            storage: Storage::new_redis(conection_manager),
        }
    }

    fn gen_id() -> String {
        let id = Uuid::new_v4();
        base62::encode(id.as_u128())
    }

    pub async fn create(&self, url: impl Into<String>) -> crate::Result<String> {
        let url = url.into();
        let id = Self::gen_id();
        debug!("generated new id {} for {}", id, url);

        self.storage.put(id.clone(), url).await?;

        Ok(id)
    }

    pub async fn get(&self, hash: impl AsRef<str>) -> crate::Result<Option<String>> {
        debug!("looking up {}", hash.as_ref());
        self.storage.get(hash).await
    }
}

#[cfg(test)]
mod test {
    use tracing_test::traced_test;

    use super::*;

    #[tokio::test]
    #[traced_test]
    async fn test_create() {
        let url = "http://test.com/";

        let shortener = UrlShortener::new_memory();

        let id = shortener.create(url).await.unwrap();
        assert!(!id.is_empty());

        let lookup = shortener.get(&id).await.unwrap();
        assert_eq!(lookup, Some(url.to_string()));
    }
}
