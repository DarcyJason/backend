use axum::{routing::post, Router};
use std::sync::Arc;

use crate::{handlers::public::auth::register, state::AppState};

pub fn public_routers() -> Router<Arc<AppState>> {
    Router::new().route("/auth/register", post(register))
}
