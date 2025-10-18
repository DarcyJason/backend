use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::custom::{
    errors::{
        access_token::AccessTokenErrorKind, device::DeviceErrorKind,
        refresh_token::RefreshTokenErrorKind, user::UserErrorKind, validation::ValidationErrorKind,
    },
    response::AppResponse,
};

pub mod access_token;
pub mod device;
pub mod from;
pub mod refresh_token;
pub mod user;
pub mod validation;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("App Configuration error: {0}")]
    AppConfigError(#[from] Box<figment::Error>),
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrorKind),
    #[error("User error: {0}")]
    UserError(#[from] UserErrorKind),
    #[error("Device error: {0}")]
    DeviceError(#[from] DeviceErrorKind),
    #[error("Access token error: {0}")]
    AccessTokenError(#[from] AccessTokenErrorKind),
    #[error("Refresh token error: {0}")]
    RefreshTokenError(#[from] RefreshTokenErrorKind),
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

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, message) = match &self {
            AppError::AppConfigError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::ValidationError(err) => match err {
                ValidationErrorKind::ValidationFailed(_) => {
                    (StatusCode::BAD_REQUEST, self.to_string())
                }
                ValidationErrorKind::PasswordHashingError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
                }
            },
            AppError::UserError(err) => match err {
                UserErrorKind::CreateUserFailed => {
                    (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
                }
                UserErrorKind::UserAlreadyExists => (StatusCode::CONFLICT, self.to_string()),
                UserErrorKind::UserNotFound => (StatusCode::NOT_FOUND, self.to_string()),
                UserErrorKind::WrongPassword => (StatusCode::UNAUTHORIZED, self.to_string()),
                UserErrorKind::TokenGenerationFailed => {
                    (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
                }
            },
            AppError::DeviceError(err) => match err {
                DeviceErrorKind::CreateDeviceFailed => {
                    (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
                }
                DeviceErrorKind::DeviceNotFound => {
                    (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
                }
            },
            AppError::AccessTokenError(err) => match err {
                AccessTokenErrorKind::AccessTokenNotFound => {
                    (StatusCode::UNAUTHORIZED, self.to_string())
                }
                AccessTokenErrorKind::InvalidAccessToken => {
                    (StatusCode::UNAUTHORIZED, self.to_string())
                }
            },
            AppError::RefreshTokenError(err) => match err {
                RefreshTokenErrorKind::CreateRefreshTokenFailed => {
                    (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
                }
                RefreshTokenErrorKind::DeleteRefreshTokenFailed => {
                    (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
                }
            },
            AppError::SurrealDBError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::RedisError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::InvalidToken(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::IOError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::OtherError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        let body = AppResponse::error(status_code.as_u16(), Some(message), ());
        (status_code, Json(body)).into_response()
    }
}
