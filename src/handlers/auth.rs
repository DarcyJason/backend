use std::sync::Arc;

use axum::{Json, extract::State};
use cookie::Cookie;
use serde_json::Value;
use validator::ValidateEmail;

use crate::{
    custom::{
        errors::{AppError, user::UserErrorKind, validation::ValidationErrorKind},
        responder::ApiResponse,
        result::AppResult,
    },
    dtos::{
        requests::{login::LoginRequest, register::RegisterRequest},
        responses::user::UserResponse,
    },
    repositories::postgres::auth::AuthRepository,
    state::AppState,
    utils::password::compare_hashed_password,
};

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Authentication Endpoint",
    request_body = RegisterRequest,
    responses(
        (
            status = 200,
            description = "Register successfully example",
            body = ApiResponse<Value>,
            example = r#"{
                "status": "success",
                "code": 200,
                "message": "users:u'dd7765ce-3858-45b8-aa8c-5067ec0ce3d4' register successfully",
                "data": null
            }"#
        ),
        (
            status = 400,
            description = "Register failed example",
            body = ApiResponse<Value>,
            example = r#"{
                "status": "error",
                "code": 400,
                "message": "Field 'name' cannot be empty",
                "data": null
            }"#
        )
    )
)]
pub async fn register_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<ApiResponse<()>> {
    if payload.name.is_empty() {
        return Err(AppError::ValidationError(ValidationErrorKind::EmptyField(
            "name".to_string(),
        )));
    }
    if payload.name.len() > 20 {
        return Err(AppError::ValidationError(ValidationErrorKind::NameTooLong));
    }
    if payload.email.is_empty() {
        return Err(AppError::ValidationError(ValidationErrorKind::EmptyField(
            "email".to_string(),
        )));
    }
    if !ValidateEmail::validate_email(&payload.email) {
        return Err(AppError::ValidationError(ValidationErrorKind::InvalidEmail));
    }
    if payload.password.is_empty() {
        return Err(AppError::ValidationError(ValidationErrorKind::EmptyField(
            "password".to_string(),
        )));
    }
    if payload.password.len() < 8 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::PasswordTooShort,
        ));
    }
    if payload.password.len() > 20 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::PasswordTooLong,
        ));
    }
    if payload.confirm_password.is_empty() {
        return Err(AppError::ValidationError(ValidationErrorKind::EmptyField(
            "confirm_password".to_string(),
        )));
    }
    if payload.confirm_password.len() < 8 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::PasswordTooShort,
        ));
    }
    if payload.confirm_password.len() > 20 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::PasswordTooLong,
        ));
    }
    if payload.password != payload.confirm_password {
        return Err(AppError::ValidationError(
            ValidationErrorKind::PasswordAndConfirmPasswordAreNotMatch,
        ));
    }
    if app_state
        .db_client
        .surreal_client
        .find_user_by_email(payload.email.clone())
        .await?
        .is_some()
    {
        return Err(AppError::UserError(UserErrorKind::UserAlreadyExists));
    }
    let user = app_state
        .db_client
        .surreal_client
        .create_user(payload.name, payload.email, payload.password)
        .await?;
    Ok(ApiResponse::success(
        format!("{} registers successfully", user.id),
        (),
    ))
}

#[utoipa::path(
    post,
    path = "/auth/Login",
    tag = "Authentication Endpoint",
    request_body = RegisterRequest,
    responses(
        (
            status = 200,
            description = "Login successfully example",
            body = ApiResponse<Value>,
            example = r#"{
                "status": "success",
                "code": 200,
                "message": "users:u'dd7765ce-3858-45b8-aa8c-5067ec0ce3d4' register successfully",
                "data": null
            }"#
        ),
        (
            status = 400,
            description = "Login failed example",
            body = ApiResponse<Value>,
            example = r#"{
                "status": "error",
                "code": 400,
                "message": "Field 'name' cannot be empty",
                "data": null
            }"#
        )
    )
)]
pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<ApiResponse<UserResponse>> {
    if payload.email.is_empty() {
        return Err(AppError::ValidationError(ValidationErrorKind::EmptyField(
            "email".to_string(),
        )));
    }
    if !ValidateEmail::validate_email(&payload.email) {
        return Err(AppError::ValidationError(ValidationErrorKind::InvalidEmail));
    }
    if payload.password.is_empty() {
        return Err(AppError::ValidationError(ValidationErrorKind::EmptyField(
            "password".to_string(),
        )));
    }
    let user = match app_state
        .db_client
        .surreal_client
        .find_user_by_email(payload.email.clone())
        .await?
    {
        Some(user) => user,
        None => return Err(AppError::UserError(UserErrorKind::UserNotFound)),
    };
    if !compare_hashed_password(payload.password, user.password.clone())? {
        return Err(AppError::UserError(UserErrorKind::WrongPassword));
    }
    let access_token = "generated_access_token".to_string();
    let refresh_token = "generated_refresh_token".to_string();
    let access_cookie = Cookie::build(("access_token", access_token))
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(time::Duration::minutes(15))
        .build();
    let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(time::Duration::days(7))
        .build();
    Ok(ApiResponse::success(
        format!("{} Login successfully", user.id),
        UserResponse::from(user),
    )
    .with_cookies(vec![access_cookie, refresh_cookie]))
}
