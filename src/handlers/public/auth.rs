use std::{net::SocketAddr, sync::Arc};

use axum::{
    Json,
    extract::{ConnectInfo, OriginalUri, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use tracing::{error, info, instrument};
use validator::ValidateEmail;

use crate::{
    constants_and_statics::regex::NAME_REGEX,
    custom::{
        errors::{
            AppError, refresh_token::RefreshTokenErrorKind, trusted_device::TrustedDeviceErrorKind,
            user::UserErrorKind, validation::ValidationErrorKind,
        },
        result::AppResult,
    },
    dtos::{
        requests::{login::LoginRequest, register::RegisterRequest},
        responses::{login::LoginResponse, register::RegisterResponse},
    },
    repositories::surreal::{
        auth::AuthRepository, refresh_token::RefreshTokenRepository,
        trusted_device::TrustedDeviceRepository,
    },
    security::{
        crypto::password::{compare_hashed_password, validate_password},
        jwt::generate_tokens,
    },
    state::AppState,
    utils::device::get_device_info,
};

#[instrument(skip(app_state, headers))]
pub async fn register(
    State(app_state): State<Arc<AppState>>,
    uri: OriginalUri,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    info!("Start handling user registration");
    if payload.name.is_empty() {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Name cannot be empty".to_string()),
        ));
    }
    if payload.name.len() > 20 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Name cannot be longer than 20 characters".to_string(),
            ),
        ));
    }
    if !NAME_REGEX.is_match(&payload.name.to_string()) {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Name must be letters, numbers or letters with numbers".to_string(),
            ),
        ));
    }
    if payload.email.is_empty() {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Email cannot be empty".to_string()),
        ));
    }
    if !ValidateEmail::validate_email(&payload.email.to_string()) {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Email must be a valid email address".to_string(),
            ),
        ));
    }
    if payload.password.is_empty() {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Password cannot be empty".to_string()),
        ));
    }
    if payload.password.len() < 8 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Password must be at least 8 characters long".to_string(),
            ),
        ));
    }
    if payload.password.len() > 20 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Password must be at most 20 characters long".to_string(),
            ),
        ));
    }
    if !validate_password(&payload.password) {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Password must contain letters, numbers and special characters".to_string(),
            ),
        ));
    }
    if payload.password != payload.confirm_password {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Passwords do not match".to_string()),
        ));
    }
    if payload.confirm_password.is_empty() {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Confirm password cannot be empty".to_string()),
        ));
    }
    if payload.confirm_password.len() < 8 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Confirm password must be at least 8 characters long".to_string(),
            ),
        ));
    }
    if payload.confirm_password.len() > 20 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Confirm password must be at most 20 characters long".to_string(),
            ),
        ));
    }
    if !validate_password(&payload.confirm_password) {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Confirm password must contain letters, numbers and special characters".to_string(),
            ),
        ));
    }
    if app_state
        .db_client
        .surreal_client
        .find_user_by_email(&payload.email)
        .await?
        .is_some()
    {
        error!("Failed: user already exists with email {}", payload.email);
        return Err(AppError::UserError(UserErrorKind::UserAlreadyExists));
    }
    match app_state
        .db_client
        .surreal_client
        .create_user(&payload.name, &payload.email, &payload.password)
        .await
    {
        Ok(_) => info!("Create user successfully"),
        Err(_) => {
            error!("Create user failed");
            return Err(AppError::UserError(UserErrorKind::CreateUserFailed));
        }
    }
    info!("Start creating trusted device");
    let device_info = get_device_info(headers);
    let user = app_state
        .db_client
        .surreal_client
        .find_user_by_email(&payload.email)
        .await
        .unwrap()
        .unwrap();
    match app_state
        .db_client
        .surreal_client
        .create_trusted_device(&user.id.id.to_raw(), device_info, addr.ip().to_string())
        .await
    {
        Ok(_) => info!("Create trusted device successfully"),
        Err(e) => {
            error!(
                "Failed: Can't create trusted device with email {}: {}",
                payload.email.clone(),
                e
            );
            return Err(AppError::TrustedDeviceError(
                TrustedDeviceErrorKind::CreateTrustedDeviceFailed,
            ));
        }
    }
    info!(
        "Finish Handling user registration successfully with email: {}",
        payload.email
    );
    Ok((
        StatusCode::OK,
        Json(RegisterResponse {
            status: "ok".to_string(),
            code: StatusCode::OK.as_u16(),
            message: "Register success".to_string(),
        }),
    )
        .into_response())
}

#[instrument(skip(app_state, headers))]
pub async fn login(
    State(app_state): State<Arc<AppState>>,
    uri: OriginalUri,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    info!("Start handling user login");
    if payload.email.is_empty() {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Email cannot be empty".to_string()),
        ));
    }
    if !ValidateEmail::validate_email(&payload.email.to_string()) {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Email must be a valid email address".to_string(),
            ),
        ));
    }
    if payload.password.is_empty() {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Password cannot be empty".to_string()),
        ));
    }
    if payload.password.len() < 8 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Password must be at least 8 characters long".to_string(),
            ),
        ));
    }
    if payload.password.len() > 20 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Password must be at most 20 characters long".to_string(),
            ),
        ));
    }
    if !validate_password(&payload.password) {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Password must contain letters, numbers and special characters".to_string(),
            ),
        ));
    }
    let user = match app_state
        .db_client
        .surreal_client
        .find_user_by_email(&payload.email)
        .await?
    {
        Some(user) => user,
        None => return Err(AppError::UserError(UserErrorKind::UserNotFound)),
    };
    if compare_hashed_password(&payload.password, &user.password)? {
        return Err(AppError::UserError(UserErrorKind::WrongPassword));
    }
    let _device_info = get_device_info(headers);
    let _user_trusted_device = app_state
        .db_client
        .surreal_client
        .find_trusted_device_by_email(&user.email)
        .await?
        .device;
    let (access_token, refresh_token) = generate_tokens(
        user.id.id.to_raw(),
        &app_state.config.jwt_config.jwt_secret.as_bytes(),
        app_state.config.jwt_config.access_token_expires_in_seconds,
        app_state.config.jwt_config.refresh_token_expires_in_seconds,
    )?;
    match app_state
        .db_client
        .surreal_client
        .create_refresh_token(&user.id.id.to_raw(), refresh_token.as_str())
        .await
    {
        Ok(_) => {
            info!("Login success with email {}", &user.email);
            Ok((
                StatusCode::OK,
                Json(LoginResponse { access_token }).into_response(),
            ))
        }
        Err(e) => {
            error!("Create refresh_token with email {} : {}", &user.email, e);
            return Err(AppError::RefreshTokenError(
                RefreshTokenErrorKind::CreateRefreshTokenFailed,
            ));
        }
    }
}
