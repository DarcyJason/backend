use crate::{
    config::redis_server::RedisServerConfig, core::error::external::ExternalError,
    core::result::AppResult,
};
use redis::aio::MultiplexedConnection;

#[derive(Debug, Clone)]
pub struct RedisClient {
    pub conn: MultiplexedConnection,
}

impl RedisClient {
    pub async fn new(redis_server_config: RedisServerConfig) -> AppResult<Self> {
        let client =
            redis::Client::open(redis_server_config.redis_address).map_err(ExternalError::from)?;
        let conn = client
            .get_multiplexed_async_connection()
            .await
            .map_err(ExternalError::from)?;
        Ok(RedisClient { conn })
    }
}
