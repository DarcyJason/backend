use axum::http::StatusCode;
use thiserror::Error;

use crate::custom::errors::{user::UserErrorKind, validation::ValidationErrorKind};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] Box<figment::Error>),
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrorKind),
    #[error("User error: {0}")]
    UserError(#[from] UserErrorKind),
    #[error("SurrealDB error: {0}")]
    SurrealDBError(#[from] Box<surrealdb::Error>),
    #[error("Redis error: {0}")]
    RedisError(#[from] Box<redis::RedisError>),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Other error: {0}")]
    OtherError(#[from] Box<anyhow::Error>),
}

impl AppError {
    pub fn status_code(&self) -> (StatusCode, String) {
        match self {
            AppError::ConfigError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::ValidationError(err) => match err {
                ValidationErrorKind::NameTooLong => (StatusCode::BAD_REQUEST, err.to_string()),
                ValidationErrorKind::InvalidEmail => (StatusCode::BAD_REQUEST, err.to_string()),
                ValidationErrorKind::EmptyField(_) => (StatusCode::BAD_REQUEST, err.to_string()),
                ValidationErrorKind::PasswordTooShort => (StatusCode::BAD_REQUEST, err.to_string()),
                ValidationErrorKind::PasswordTooLong => (StatusCode::BAD_REQUEST, err.to_string()),
                ValidationErrorKind::ConfirmPasswordTooShort => {
                    (StatusCode::BAD_REQUEST, err.to_string())
                }
                ValidationErrorKind::ConfirmPasswordTooLong => {
                    (StatusCode::BAD_REQUEST, err.to_string())
                }
                ValidationErrorKind::PasswordAndConfirmPasswordAreNotMatch => {
                    (StatusCode::BAD_REQUEST, err.to_string())
                }
                ValidationErrorKind::PasswordHashingError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
            },
            AppError::UserError(err) => match err {
                UserErrorKind::CreateUserFailed => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
                UserErrorKind::UserAlreadyExists => (StatusCode::BAD_REQUEST, err.to_string()),
                UserErrorKind::UserNotFound => (StatusCode::NOT_FOUND, err.to_string()),
            },
            AppError::SurrealDBError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::RedisError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::IOError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::OtherError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        }
    }
}
