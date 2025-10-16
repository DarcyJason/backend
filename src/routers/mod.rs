use std::sync::Arc;
use axum::Router;

use crate::{routers::{protected::protected_routers, public::public_routers}, state::AppState};

pub mod protected;
pub mod public;

pub fn api_routers() -> Router<Arc<AppState>> {
    let all_router = Router::new()
        .merge(public_routers())
        .merge(protected_routers());
    Router::new().nest("/api/v1", all_router)
}
