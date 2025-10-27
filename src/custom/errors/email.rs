use axum::http::StatusCode;
use thiserror::Error;

use crate::custom::errors::error_trait::ErrorKind;

#[derive(Debug, Error)]
pub enum EmailErrorKind {
    #[error("Create email failed")]
    CreateEmailFailed,
    #[error("Email not found")]
    EmailNotFound,
    #[error("Invalid token")]
    InvalidToken,
}

impl ErrorKind for EmailErrorKind {
    fn status_code(&self) -> StatusCode {
        match self {
            EmailErrorKind::CreateEmailFailed => StatusCode::INTERNAL_SERVER_ERROR,
            EmailErrorKind::EmailNotFound => StatusCode::NOT_FOUND,
            EmailErrorKind::InvalidToken => StatusCode::UNAUTHORIZED,
        }
    }
    fn message(&self) -> String {
        self.to_string()
    }
}
