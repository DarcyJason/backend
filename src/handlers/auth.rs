use std::{net::SocketAddr, sync::Arc};

use axum::{
    Json,
    extract::{ConnectInfo, OriginalUri, State},
    http::{HeaderMap, header::AUTHORIZATION},
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use tracing::{error, info, instrument};
use validator::ValidateEmail;

use crate::{
    custom::{
        errors::{
            AppError, refresh_token::RefreshTokenErrorKind, user::UserErrorKind,
            validation::ValidationErrorKind,
        },
        response::AppResponse,
        result::AppResult,
    },
    dtos::requests::{login::LoginRequest, register::RegisterRequest},
    repositories::surreal::{auth::AuthRepository, refresh_token::RefreshTokenRepository},
    state::AppState,
    statics::regex::NAME_REGEX,
    utils::{
        password::{compare_hashed_password, validate_password},
        token::{generate_access_token, generate_refresh_token},
    },
};

#[instrument(skip(app_state))]
pub async fn register(
    State(app_state): State<Arc<AppState>>,
    uri: OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    info!("✅ Start handling user registration");
    if payload.name.is_empty() {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Name can't be empty".to_string()),
        ));
    }
    if payload.name.len() > 20 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Name can't be longer than 20 characters".to_string(),
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
            ValidationErrorKind::ValidationFailed("Password can't be empty".to_string()),
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
        error!(
            "❌ Failed: user already exists with email {}",
            payload.email
        );
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
    info!("✅ Start creating trusted device");
    info!(
        "✅ Finish Handling user registration successfully with email: {}",
        payload.email
    );
    Ok(AppResponse::success("Register success".to_string(), ()))
}

#[instrument(skip(app_state))]
pub async fn login(
    State(app_state): State<Arc<AppState>>,
    uri: OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    info!("✅ Start handling user login");
    if payload.email.is_empty() {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Email can't be empty".to_string()),
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
            ValidationErrorKind::ValidationFailed("Password can't be empty".to_string()),
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
    if !compare_hashed_password(&payload.password, &user.password)? {
        return Err(AppError::UserError(UserErrorKind::WrongPassword));
    }
    let access_token = generate_access_token(
        user.id.id.to_raw(),
        app_state.config.jwt_config.jwt_secret.as_bytes(),
        app_state.config.jwt_config.access_token_expires_in_seconds,
    )?;
    let refresh_token = generate_refresh_token();
    match app_state
        .db_client
        .surreal_client
        .create_refresh_token(&user.id.id.to_raw(), &refresh_token)
        .await
    {
        Ok(_) => {
            info!("✅ Start setting access_token and refresh_token in response");
            let mut headers = HeaderMap::new();
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", access_token).parse().unwrap(),
            );
            let cookie = Cookie::build(("refresh_token", refresh_token))
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Strict)
                .build();
            let updated_jar = CookieJar::new().add(cookie);
            info!("✅ Login success with email {}", &user.email);
            Ok((
                headers,
                updated_jar,
                AppResponse::success("Login Success".to_string(), ()),
            ))
        }
        Err(e) => {
            error!("❌ Create refresh_token with email {} : {}", &user.email, e);
            return Err(AppError::RefreshTokenError(
                RefreshTokenErrorKind::CreateRefreshTokenFailed,
            ));
        }
    }
}

#[instrument]
pub async fn forget_password() {}

#[instrument]
pub async fn reset_password() {}
