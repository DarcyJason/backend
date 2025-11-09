use std::sync::Arc;

use crate::{core::result::AppResult, core::state::AppState};
use axum::{extract::State, response::IntoResponse};

pub async fn health_check(State(app_state): State<Arc<AppState>>) -> AppResult<impl IntoResponse> {
    app_state.health_service.health_check().await
}
