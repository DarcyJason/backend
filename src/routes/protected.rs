use crate::state::AppState;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

pub fn protected_routes() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
}
