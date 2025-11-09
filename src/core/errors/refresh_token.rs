use axum::http::StatusCode;
use thiserror::Error;

use crate::core::errors::error_trait::ErrorKind;

#[derive(Debug, Error)]
pub enum RefreshTokenErrorKind {
    #[error("Create refresh token failed")]
    CreateRefreshTokenFailed,
    #[error("Delete refresh token failed")]
    DeleteRefreshTokenFailed,
}

impl ErrorKind for RefreshTokenErrorKind {
    fn status_code(&self) -> StatusCode {
        match self {
            RefreshTokenErrorKind::CreateRefreshTokenFailed => StatusCode::INTERNAL_SERVER_ERROR,
            RefreshTokenErrorKind::DeleteRefreshTokenFailed => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn message(&self) -> String {
        self.to_string()
    }
}
