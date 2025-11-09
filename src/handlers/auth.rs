use axum::{
    Extension, Json,
    extract::{ConnectInfo, OriginalUri, State},
    http::HeaderMap,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use std::{net::SocketAddr, sync::Arc};
use tracing::instrument;

use crate::dto::auth::LoginDTO;
use crate::{
    core::result::AppResult,
    core::state::AppState,
    dto::auth::{ForgetPasswordDTO, RegisterDTO, ResetPasswordDTO, VerifyUserDTO},
    models::user::User,
};

#[instrument(skip(app_state))]
pub async fn register(
    State(app_state): State<Arc<AppState>>,
    uri: OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<RegisterDTO>,
) -> AppResult<impl IntoResponse> {
    app_state.auth_service.register(payload).await
}

#[instrument(skip(app_state, headers, jar))]
pub async fn login(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
    jar: CookieJar,
    uri: OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<LoginDTO>,
) -> AppResult<impl IntoResponse> {
    app_state.auth_service.login(headers, jar, payload).await
}

#[instrument(skip(app_state, jar, user))]
pub async fn logout(
    State(app_state): State<Arc<AppState>>,
    jar: CookieJar,
    Extension(user): Extension<User>,
) -> AppResult<impl IntoResponse> {
    app_state.auth_service.logout(jar, user).await
}

#[instrument(skip(app_state, headers))]
pub async fn verify_email(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<VerifyUserDTO>,
) -> AppResult<impl IntoResponse> {
    app_state
        .auth_service
        .verify_email(headers, addr, payload)
        .await
}

#[instrument(skip(app_state))]
pub async fn forget_password(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<ForgetPasswordDTO>,
) -> AppResult<impl IntoResponse> {
    app_state.auth_service.forget_password(payload).await
}

#[instrument(skip(app_state))]
pub async fn reset_password(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<ResetPasswordDTO>,
) -> AppResult<impl IntoResponse> {
    app_state.auth_service.reset_password(payload).await
}
