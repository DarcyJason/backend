use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    handlers::auth::{login, register},
    state::AppState,
};

pub fn auth_routers(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .with_state(app_state)
}
