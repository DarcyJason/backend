use axum::Router;
use std::sync::Arc;

use crate::{
    routers::{auth::auth_routers, user::user_routers},
    state::AppState,
};

pub mod auth;
pub mod user;

pub fn api_routers(app_state: Arc<AppState>) -> Router {
    let all_router = Router::new()
        .merge(auth_routers(app_state.clone()))
        .merge(user_routers(app_state));
    Router::new().nest("/api/v1", all_router)
}
