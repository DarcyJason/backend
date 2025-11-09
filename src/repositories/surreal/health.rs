use async_trait::async_trait;

use crate::{core::result::AppResult, database::surreal::client::SurrealClient};

#[async_trait]
pub trait HealthRepository {
    async fn health_check(&self) -> AppResult<bool>;
}

#[async_trait]
impl HealthRepository for SurrealClient {
    async fn health_check(&self) -> AppResult<bool> {
        let sql = "RETURN time::now()";
        match self.client.query(sql).await {
            Ok(mut result) => match result.take::<Option<String>>(0) {
                Ok(Some(_)) => Ok(true),
                _ => Ok(false),
            },
            Err(_) => Ok(false),
        }
    }
}
