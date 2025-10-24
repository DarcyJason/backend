use validator::ValidateEmail;

use crate::{
    custom::{
        errors::{AppError, validation::ValidationErrorKind},
        result::AppResult,
    },
    dtos::requests::auth::{
        ForgetPasswordRequest, LoginRequest, RegisterRequest, ResetPasswordRequest,
        VerifyUserRequest,
    },
    lazy::regex::NAME_REGEX,
    utils::password::validate_password,
};

pub fn validate_register_payload(payload: &RegisterRequest) -> AppResult<()> {
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
    if payload.password != payload.confirm_password {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Passwords do not match".to_string()),
        ));
    }
    Ok(())
}

pub fn validate_login_payload(payload: &LoginRequest) -> AppResult<()> {
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
    Ok(())
}

pub fn validate_verify_user_payload(payload: &VerifyUserRequest) -> AppResult<()> {
    if payload.token.is_empty() {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Token can't be empty".to_string()),
        ));
    }
    if !ValidateEmail::validate_email(&payload.email.to_string()) {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Email must be a valid email address".to_string(),
            ),
        ));
    }
    Ok(())
}

pub fn validate_forget_password_payload(payload: &ForgetPasswordRequest) -> AppResult<()> {
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
    Ok(())
}

pub fn validate_reset_password_payload(payload: &ResetPasswordRequest) -> AppResult<()> {
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
    if payload.token.is_empty() {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Token can't be empty".to_string()),
        ));
    }
    if payload.new_password.is_empty() {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Password can't be empty".to_string()),
        ));
    }
    if payload.new_password.len() < 8 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Password must be at least 8 characters long".to_string(),
            ),
        ));
    }
    if payload.new_password.len() > 20 {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Password must be at most 20 characters long".to_string(),
            ),
        ));
    }
    if !validate_password(&payload.new_password) {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed(
                "Password must contain letters, numbers and special characters".to_string(),
            ),
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
    if payload.new_password != payload.confirm_password {
        return Err(AppError::ValidationError(
            ValidationErrorKind::ValidationFailed("Passwords do not match".to_string()),
        ));
    }
    Ok(())
}
