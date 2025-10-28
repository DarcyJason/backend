use axum::{
    Extension, Json,
    extract::{ConnectInfo, OriginalUri, State},
    http::HeaderMap,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use std::{net::SocketAddr, sync::Arc};
use tracing::instrument;

use crate::dtos::requests::auth::LoginRequest;
use crate::{
    core::app_state::AppState,
    custom::result::AppResult,
    dtos::requests::auth::{
        ForgetPasswordRequest, RegisterRequest, ResetPasswordRequest, VerifyUserRequest,
    },
    models::user::User,
};

#[instrument(skip(app_state))]
pub async fn register(
    State(app_state): State<Arc<AppState>>,
    uri: OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<RegisterRequest>,
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
    Json(payload): Json<LoginRequest>,
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
pub async fn verify_user(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<VerifyUserRequest>,
) -> AppResult<impl IntoResponse> {
    app_state
        .auth_service
        .verify_user(headers, addr, payload)
        .await
}

#[instrument(skip(app_state))]
pub async fn forget_password(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<ForgetPasswordRequest>,
) -> AppResult<impl IntoResponse> {
    app_state.auth_service.forget_password(payload).await
}

#[instrument(skip(app_state))]
pub async fn reset_password(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<ResetPasswordRequest>,
) -> AppResult<impl IntoResponse> {
    app_state.auth_service.reset_password(payload).await
}
