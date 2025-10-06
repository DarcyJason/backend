use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::custom::errors::{
    refresh_token::RefreshTokenErrorKind, trusted_device::TrustedDeviceErrorKind,
    user::UserErrorKind, validation::ValidationErrorKind,
};

pub mod from;
pub mod refresh_token;
pub mod trusted_device;
pub mod user;
pub mod validation;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] Box<figment::Error>),
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrorKind),
    #[error("User error: {0}")]
    UserError(#[from] UserErrorKind),
    #[error("Trusted device error: {0}")]
    TrustedDeviceError(#[from] TrustedDeviceErrorKind),
    #[error("Refresh token error: {0}")]
    RefreshTokenError(#[from] RefreshTokenErrorKind),
    #[error("Email error: {0}")]
    EmailError(#[from] lettre::error::Error),
    #[error("Email address error: {0}")]
    EmailAddressError(#[from] lettre::address::AddressError),
    #[error("Email transport error: {0}")]
    EmailTransportError(#[from] lettre::transport::smtp::Error),
    #[error("SurrealDB error: {0}")]
    SurrealDBError(#[from] Box<surrealdb::Error>),
    #[error("Redis error: {0}")]
    RedisError(#[from] Box<redis::RedisError>),
    #[error("Token error")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Other error: {0}")]
    OtherError(#[from] Box<anyhow::Error>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub code: u16,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            AppError::ConfigError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::ValidationError(err) => match err {
                ValidationErrorKind::ValidationFailed(err) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
                ValidationErrorKind::PasswordHashingError(err) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
            },
            AppError::UserError(err) => match err {
                UserErrorKind::CreateUserFailed => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
                UserErrorKind::UserAlreadyExists => (StatusCode::BAD_REQUEST, err.to_string()),
                UserErrorKind::UserNotFound => (StatusCode::NOT_FOUND, err.to_string()),
                UserErrorKind::WrongPassword => (StatusCode::UNAUTHORIZED, err.to_string()),
                UserErrorKind::TokenGenerationFailed => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
            },
            AppError::TrustedDeviceError(err) => match err {
                TrustedDeviceErrorKind::CreateTrustedDeviceFailed => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
                TrustedDeviceErrorKind::TrustedDeviceNotFound => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
            },
            AppError::RefreshTokenError(err) => match err {
                RefreshTokenErrorKind::CreateRefreshTokenFailed => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
            },
            AppError::EmailError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::EmailAddressError(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            AppError::EmailTransportError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
            AppError::SurrealDBError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::RedisError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::InvalidToken(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::IOError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::OtherError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };
        let body = ErrorResponse {
            status: "err".to_string(),
            code: status_code.as_u16(),
            message,
        };
        (status_code, Json(body)).into_response()
    }
}
