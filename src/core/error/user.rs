use axum::http::StatusCode;
use thiserror::Error;

use crate::core::error::error_trait::ErrorKind;

#[derive(Debug, Error)]
pub enum UserErrorKind {
    #[error("Failed to create user")]
    CreateUserFailed,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Wrong password")]
    WrongPassword,
    #[error("Password must be different from last password")]
    PasswordMustBeDifferentFromLastPassword,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Failed to generate tokens")]
    TokenGenerationFailed,
    #[error("Missing user agent")]
    MissingUserAgent,
}

impl ErrorKind for UserErrorKind {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::CreateUserFailed => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UserAlreadyExists => StatusCode::CONFLICT,
            Self::UserNotFound => StatusCode::NOT_FOUND,
            Self::WrongPassword => StatusCode::UNAUTHORIZED,
            Self::PasswordMustBeDifferentFromLastPassword => StatusCode::UNAUTHORIZED,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::TokenGenerationFailed => StatusCode::INTERNAL_SERVER_ERROR,
            Self::MissingUserAgent => StatusCode::BAD_REQUEST,
        }
    }
    fn message(&self) -> String {
        self.to_string()
    }
}
