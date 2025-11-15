use std::sync::Arc;

use axum::{Extension, Json, extract::State, response::IntoResponse};
use tracing::instrument;

use crate::{
    core::{result::AppResult, state::AppState},
    dto::request::user::ChangePasswordRequest,
    models::user::User,
};

#[instrument(skip(app_state))]
pub async fn get_me(
    State(app_state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> AppResult<impl IntoResponse> {
    app_state.services.user.get_me(user).await
}

#[instrument(skip(app_state))]
pub async fn change_password(
    State(app_state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<ChangePasswordRequest>,
) -> AppResult<impl IntoResponse> {
    app_state.services.user.change_password(user, payload).await
}

pub async fn list_devices() {}

pub async fn delete_account() {}
