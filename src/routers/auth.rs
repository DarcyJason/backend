use std::sync::Arc;

use axum::{Router, middleware, routing::post};

use crate::{
    handlers::auth::{forget_password, login, logout, register, reset_password, verify_user},
    middlewares::auth::{auth, role_check},
    models::user::UserRole,
    state::AppState,
};

pub fn auth_routers(app_state: Arc<AppState>) -> Router {
    let auth_routers = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route(
            "/logout",
            post(logout)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
                .route_layer(middleware::from_fn(|req, next| {
                    role_check(req, next, vec![UserRole::Admin, UserRole::User])
                })),
        )
        .route("/verify-user", post(verify_user))
        .route("/forget-password", post(forget_password))
        .route("/reset-password", post(reset_password))
        .with_state(app_state);
    Router::new().nest("/auth", auth_routers)
}
