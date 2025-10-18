use std::sync::Arc;

use axum::{Router, middleware, routing::get};

use crate::{handlers::user::current_user, middlewares::auth::auth, state::AppState};

pub fn user_routers(app_state: Arc<AppState>) -> Router {
    let user_router = Router::new().route("/me", get(current_user));
    Router::new().nest(
        "/user",
        user_router.layer(middleware::from_fn_with_state(app_state, auth)),
    )
}
