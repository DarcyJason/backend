use std::sync::Arc;

use axum::{Router, middleware, routing::post};

use crate::{
    handlers::auth::{login, logout, register},
    middlewares::auth::{auth, role_check},
    models::user::UserRole,
    state::AppState,
};

pub fn auth_routers(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route(
            "/auth/logout",
            post(logout)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
                .route_layer(middleware::from_fn(|req, next| {
                    role_check(req, next, vec![UserRole::Admin, UserRole::User])
                })),
        )
        .with_state(app_state)
}
