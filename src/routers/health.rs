use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{core::state::AppState, handlers::health::health_check};

pub fn health_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(app_state)
}
