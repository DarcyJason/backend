use crate::{
    core::{error::validation::ValidationErrorKind, result::AppResult},
    dto::request::user::ChangePasswordRequest,
    utils::regex::PASSWORD_REGEX,
};

pub fn validate_change_password_request(payload: &ChangePasswordRequest) -> AppResult<()> {
    if payload.old_password.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("Password can't be empty".to_string()).into(),
        );
    }
    if payload.old_password.len() < 8 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password must be at least 8 characters long".to_string(),
        )
        .into());
    }
    if payload.old_password.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password must be at most 20 characters long".to_string(),
        )
        .into());
    }
    if !PASSWORD_REGEX
        .is_match(&payload.old_password)
        .unwrap_or(false)
    {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password must contain letters, numbers and special characters".to_string(),
        )
        .into());
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
    if payload.new_confirm_password.is_empty() {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password cannot be empty".to_string(),
        )
        .into());
    }
    if payload.new_confirm_password.len() < 8 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password must be at least 8 characters long".to_string(),
        )
        .into());
    }
    if payload.new_confirm_password.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password must be at most 20 characters long".to_string(),
        )
        .into());
    }
    if !PASSWORD_REGEX
        .is_match(&payload.new_confirm_password)
        .unwrap_or(false)
    {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password must contain letters, numbers and special characters".to_string(),
        )
        .into());
    }
    if payload.new_password != payload.new_confirm_password {
        return Err(
            ValidationErrorKind::ValidationFailed("Passwords do not match".to_string()).into(),
        );
    }
    Ok(())
}
