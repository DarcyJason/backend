use validator::ValidateEmail;

use crate::{
    custom::{errors::validation::ValidationErrorKind, result::AppResult},
    dto::auth::{
        ForgetPasswordDTO, LoginDTO, RegisterDTO, ResetPasswordDTO,
        VerifyUserDTO,
    },
    lazy::regex::{NAME_REGEX, PASSWORD_REGEX},
};

pub fn validate_register_payload(payload: &RegisterDTO) -> AppResult<()> {
    if payload.name.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("Name can't be empty".to_string()).into(),
        );
    }
    if payload.name.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Name can't be longer than 20 characters".to_string(),
        )
        .into());
    }
    if !NAME_REGEX.is_match(&payload.name).unwrap_or(false) {
        return Err(ValidationErrorKind::ValidationFailed(
            "Name must be letters, numbers or letters with numbers".to_string(),
        )
        .into());
    }
    if payload.email.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("Email cannot be empty".to_string()).into(),
        );
    }
    if !ValidateEmail::validate_email(&payload.email) {
        return Err(ValidationErrorKind::ValidationFailed(
            "Email must be a valid email address".to_string(),
        )
        .into());
    }
    if payload.password.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("Password can't be empty".to_string()).into(),
        );
    }
    if payload.password.len() < 8 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password must be at least 8 characters long".to_string(),
        )
        .into());
    }
    if payload.password.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password must be at most 20 characters long".to_string(),
        )
        .into());
    }
    if !PASSWORD_REGEX.is_match(&payload.password).unwrap_or(false) {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password must contain letters, numbers and special characters".to_string(),
        )
        .into());
    }
    if payload.confirm_password.is_empty() {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password cannot be empty".to_string(),
        )
        .into());
    }
    if payload.confirm_password.len() < 8 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password must be at least 8 characters long".to_string(),
        )
        .into());
    }
    if payload.confirm_password.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password must be at most 20 characters long".to_string(),
        )
        .into());
    }
    if !PASSWORD_REGEX
        .is_match(&payload.confirm_password)
        .unwrap_or(false)
    {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password must contain letters, numbers and special characters".to_string(),
        )
        .into());
    }
    if payload.password != payload.confirm_password {
        return Err(
            ValidationErrorKind::ValidationFailed("Passwords do not match".to_string()).into(),
        );
    }
    Ok(())
}

pub fn validate_login_payload(payload: &LoginDTO) -> AppResult<()> {
    if payload.email.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("Email can't be empty".to_string()).into(),
        );
    }
    if !ValidateEmail::validate_email(&payload.email) {
        return Err(ValidationErrorKind::ValidationFailed(
            "Email must be a valid email address".to_string(),
        )
        .into());
    }
    if payload.password.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("Password can't be empty".to_string()).into(),
        );
    }
    if payload.password.len() < 8 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password must be at least 8 characters long".to_string(),
        )
        .into());
    }
    if payload.password.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password must be at most 20 characters long".to_string(),
        )
        .into());
    }
    if !PASSWORD_REGEX.is_match(&payload.password).unwrap_or(false) {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password must contain letters, numbers and special characters".to_string(),
        )
        .into());
    }
    Ok(())
}

pub fn validate_verify_user_payload(payload: &VerifyUserDTO) -> AppResult<()> {
    if payload.email_token.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("Token can't be empty".to_string()).into(),
        );
    }
    if !ValidateEmail::validate_email(&payload.email) {
        return Err(ValidationErrorKind::ValidationFailed(
            "Email must be a valid email address".to_string(),
        )
        .into());
    }
    Ok(())
}

pub fn validate_forget_password_payload(payload: &ForgetPasswordDTO) -> AppResult<()> {
    if payload.email.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("Email can't be empty".to_string()).into(),
        );
    }
    if !ValidateEmail::validate_email(&payload.email) {
        return Err(ValidationErrorKind::ValidationFailed(
            "Email must be a valid email address".to_string(),
        )
        .into());
    }
    Ok(())
}

pub fn validate_reset_password_payload(payload: &ResetPasswordDTO) -> AppResult<()> {
    if payload.email.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("Email can't be empty".to_string()).into(),
        );
    }
    if !ValidateEmail::validate_email(&payload.email) {
        return Err(ValidationErrorKind::ValidationFailed(
            "Email must be a valid email address".to_string(),
        )
        .into());
    }
    if payload.token.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("Token can't be empty".to_string()).into(),
        );
    }
    if payload.new_password.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("Password can't be empty".to_string()).into(),
        );
    }
    if payload.new_password.len() < 8 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password must be at least 8 characters long".to_string(),
        )
        .into());
    }
    if payload.new_password.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password must be at most 20 characters long".to_string(),
        )
        .into());
    }
    if !PASSWORD_REGEX
        .is_match(&payload.new_password)
        .unwrap_or(false)
    {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password must contain letters, numbers and special characters".to_string(),
        )
        .into());
    }
    if payload.confirm_password.is_empty() {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password cannot be empty".to_string(),
        )
        .into());
    }
    if payload.confirm_password.len() < 8 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password must be at least 8 characters long".to_string(),
        )
        .into());
    }
    if payload.confirm_password.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password must be at most 20 characters long".to_string(),
        )
        .into());
    }
    if !PASSWORD_REGEX
        .is_match(&payload.confirm_password)
        .unwrap_or(false)
    {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password must contain letters, numbers and special characters".to_string(),
        )
        .into());
    }
    if payload.new_password != payload.confirm_password {
        return Err(
            ValidationErrorKind::ValidationFailed("Passwords do not match".to_string()).into(),
        );
    }
    Ok(())
}
