use std::sync::Arc;

use anyhow::anyhow;
use axum::response::IntoResponse;

use crate::{
    config::AppConfig,
    custom::{errors::AppError, result::AppResult},
    database::client::DBClient,
    repositories::{
        redis::health::HealthRepository,
        surreal::health::HealthRepository as SurrealHealthRepository,
    },
};

#[derive(Debug)]
pub struct HealthService {
    pub config: Arc<AppConfig>,
    pub db_client: Arc<DBClient>,
}

impl HealthService {
    pub fn new(config: Arc<AppConfig>, db_client: Arc<DBClient>) -> Self {
        Self { config, db_client }
    }
    pub async fn health_check(&self) -> AppResult<impl IntoResponse + use<>> {
        let checks: (bool, bool) = tokio::join!(
            async { self.db_client.redis_client.health_check().await.is_ok() },
            async { self.db_client.surreal_client.health_check().await.is_ok() }
        );
        if !checks.0 && !checks.1 {
            Err(AppError::OtherError(Box::new(anyhow!(
                "Redis server error and SurrealDB server error"
            ))))
        } else if !checks.0 && checks.1 {
            Err(AppError::OtherError(Box::new(anyhow!(
                "Redis server error"
            ))))
        } else if checks.0 && !checks.1 {
            Err(AppError::OtherError(Box::new(anyhow!(
                "SurrealDB server error"
            ))))
        } else {
            Ok("healthy")
        }
    }
}
