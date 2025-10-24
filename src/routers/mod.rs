use axum::Router;
use std::sync::Arc;

use crate::{
    core::app_state::AppState,
    routers::{auth::auth_routers, health::health_router, user::user_routers},
};

pub mod auth;
pub mod health;
pub mod user;

pub fn api_routers(app_state: Arc<AppState>) -> Router {
    let all_router = Router::new()
        .merge(health_router(app_state.clone()))
        .merge(auth_routers(app_state.clone()))
        .merge(user_routers(app_state));
    Router::new().nest("/api/v1", all_router)
}
