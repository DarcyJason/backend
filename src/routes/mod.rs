use crate::openapi::ApiDoc;
use crate::routes::protected::protected_routes;
use crate::routes::public::public_routes;
use crate::state::AppState;
use axum::Router;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

pub mod protected;
pub mod public;

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    let (router, openapi) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1", create_api_routes())
        .with_state(app_state)
        .split_for_parts();

    router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi.clone()))
        .merge(Redoc::with_url("/redoc", openapi.clone()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", openapi))
}

pub fn create_api_routes() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .merge(public_routes())
        .merge(protected_routes())
}
