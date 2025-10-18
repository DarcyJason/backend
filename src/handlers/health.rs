use std::sync::Arc;

use crate::{
    custom::{errors::AppError, result::AppResult},
    repositories::{
        redis::health::HealthRepository,
        surreal::health::HealthRepository as SurrealHealthRepository,
    },
    state::AppState,
};
use anyhow::anyhow;
use axum::{extract::State, response::IntoResponse};

pub async fn health_check(State(app_state): State<Arc<AppState>>) -> AppResult<impl IntoResponse> {
    let checks: (bool, bool) = tokio::join!(
        async {
            app_state
                .db_client
                .redis_client
                .health_check()
                .await
                .is_ok()
        },
        async {
            app_state
                .db_client
                .surreal_client
                .health_check()
                .await
                .is_ok()
        }
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
