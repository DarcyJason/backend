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
            AppError, trusted_device::TrustedDeviceErrorKind, user::UserErrorKind,
            validation::ValidationErrorKind,
        },
        result::AppResult,
    },
    dtos::{requests::register::RegisterRequest, responses::register::RegisterResponse},
    repositories::surreal::{auth::AuthRepository, trusted_device::TrustedDeviceRepository},
    security::crypto::password::validate_password,
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
    let user_agent = headers
        .get("User-Agent")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let device_info = get_device_info(user_agent);
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
                TrustedDeviceErrorKind::CreateTrustDeviceFailed,
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
            status: "success".to_string(),
            code: StatusCode::OK.as_u16(),
            message: "Register success".to_string(),
        }),
    )
        .into_response())
}
