use std::sync::Arc;

use axum::{Router, middleware, routing::get};

use crate::{
    core::state::AppState,
    handlers::user::get_me,
    middlewares::auth::{auth, role_check},
    models::user::UserRole,
};

pub fn user_routers(app_state: Arc<AppState>) -> Router {
    let user_router = Router::new()
        .route(
            "/me",
            get(get_me).route_layer(middleware::from_fn(|req, next| {
                role_check(req, next, vec![UserRole::Admin, UserRole::User])
            })),
        )
        .layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .with_state(app_state);
    Router::new().nest("/user", user_router)
}
