use axum::Json;
use serde_json::Value;
use validator::ValidateEmail;

use crate::{
    custom::{
        errors::{app_error::AppError, validation::ValidationErrorKind},
        responder::ApiResponse,
        result::AppResult,
    },
    dtos::requests::register::RegisterRequest,
};

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Authentication Endpoint",
    request_body = RegisterRequest,
    responses(
        (
            status = 200,
            description = "Registration successful",
            body = ApiResponse<Value>,
            example = r#"{
                "status": "success",
                "code": 200,
                "message": "Registration successful.",
                "data": null
            }"#
        ),
        (
            status = 400,
            description = "Validation error (e.g., empty field, invalid email)",
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
pub async fn register_handler(Json(payload): Json<RegisterRequest>) -> AppResult<ApiResponse<()>> {
    if payload.name.is_empty() {
        return Err(AppError::ValidationError(ValidationErrorKind::EmptyField(
            "name".to_string(),
        )));
    }
    if payload.name.len() < 1 {
        return Err(AppError::ValidationError(ValidationErrorKind::NameTooShort));
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
    Ok(ApiResponse::success(
        "Registration successful.".to_string(),
        (),
    ))
}
