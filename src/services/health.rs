use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};

use crate::{
    core::error::other::OtherErrorKind,
    core::{config::AppConfig, response::AppResponse, result::AppResult},
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
            Err(
                OtherErrorKind::Error("Redis server error and SurrealDB server error".to_string())
                    .into(),
            )
        } else if !checks.0 && checks.1 {
            Err(OtherErrorKind::Error("Redis server error".to_string()).into())
        } else if checks.0 && !checks.1 {
            Err(OtherErrorKind::Error("SurrealDB server error".to_string()).into())
        } else {
            Ok(AppResponse::<()>::success(
                StatusCode::OK.as_u16(),
                "UP",
                StatusCode::OK.canonical_reason().unwrap_or("OK"),
                None,
            ))
        }
    }
}
