use async_trait::async_trait;
use redis::AsyncTypedCommands;

use crate::{
    core::result::AppResult, core::errors::external::ExternalError,
    database::redis::client::RedisClient,
};

#[async_trait]
pub trait HealthRepository {
    async fn health_check(&self) -> AppResult<bool>;
}

#[async_trait]
impl HealthRepository for RedisClient {
    async fn health_check(&self) -> AppResult<bool> {
        let mut conn = self.conn.clone();
        let response = conn.ping().await.map_err(ExternalError::from)?;
        Ok(response == "PONG")
    }
}
