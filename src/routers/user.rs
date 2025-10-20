use std::sync::Arc;

use axum::{Router, middleware, routing::get};

use crate::{
    handlers::user::get_me,
    middlewares::auth::{auth, role_check},
    models::user::UserRole,
    state::AppState,
};

pub fn user_routers(app_state: Arc<AppState>) -> Router {
    let user_router = Router::new().route(
        "/me",
        get(get_me).route_layer(middleware::from_fn(|req, next| {
            role_check(req, next, vec![UserRole::Admin, UserRole::User])
        })),
    );
    Router::new().nest(
        "/user",
        user_router.layer(middleware::from_fn_with_state(app_state, auth)),
    )
}
