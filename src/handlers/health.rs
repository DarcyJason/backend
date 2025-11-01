use std::sync::Arc;

use crate::{core::app_state::AppState, custom::result::AppResult};
use axum::{extract::State, response::IntoResponse};

pub async fn health_check(State(app_state): State<Arc<AppState>>) -> AppResult<impl IntoResponse> {
    app_state.health_service.health_check().await
}
