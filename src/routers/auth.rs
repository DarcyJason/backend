use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{
        auth::{login, register},
        health::health_check,
    },
    state::AppState,
};

pub fn auth_routers(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .with_state(app_state)
}
