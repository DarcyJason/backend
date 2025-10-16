use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use crate::{
    handlers::public::{auth::register, health::health_check},
    state::AppState,
};

pub fn public_routers() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health_check))
        .route("/auth/register", post(register))
}
