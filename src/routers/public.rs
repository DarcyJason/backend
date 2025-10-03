use std::sync::Arc;
use axum::Router;

use crate::state::AppState;

pub fn public_routers() -> Router<Arc<AppState>> {
    Router::new()
}
