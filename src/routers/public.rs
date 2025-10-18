use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use crate::{
    handlers::{
        auth::{login, register},
        health::health_check,
    },
    state::AppState,
};

pub fn public_routers() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health_check))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}
