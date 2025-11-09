use axum::http::StatusCode;
use thiserror::Error;

use crate::core::errors::error_trait::ErrorKind;

#[derive(Debug, Error)]
pub enum ValidationErrorKind {
    #[error("{0}")]
    ValidationFailed(String),
    #[error("Password hashing error: {0}")]
    PasswordHashingError(#[from] argon2::password_hash::Error),
}

impl ErrorKind for ValidationErrorKind {
    fn status_code(&self) -> axum::http::StatusCode {
        match self {
            ValidationErrorKind::ValidationFailed(_) => StatusCode::BAD_REQUEST,
            ValidationErrorKind::PasswordHashingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn message(&self) -> String {
        self.to_string()
    }
}
