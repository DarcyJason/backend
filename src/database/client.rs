use crate::config::AppConfig;
use crate::custom::result::AppResult;
use crate::database::redis::client::RedisClient;
use crate::database::surreal::client::SurrealClient;

#[derive(Debug, Clone)]
pub struct DBClient {
    pub surreal_client: SurrealClient,
    pub redis_client: RedisClient,
}

impl DBClient {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        let surreal_client = SurrealClient::new(config.surreal_server).await?;
        let redis_client = RedisClient::new(config.redis_server).await?;
        Ok(DBClient {
            surreal_client,
            redis_client,
        })
    }
}
