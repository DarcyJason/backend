use axum::http::StatusCode;
use thiserror::Error;

use crate::core::errors::error_trait::ErrorKind;

#[derive(Debug, Error)]
pub enum AccessTokenErrorKind {
    #[error("Access token not found")]
    AccessTokenNotFound,
    #[error("Invalid access token")]
    InvalidAccessToken,
}

impl ErrorKind for AccessTokenErrorKind {
    fn status_code(&self) -> StatusCode {
        match self {
            AccessTokenErrorKind::AccessTokenNotFound => StatusCode::NOT_FOUND,
            AccessTokenErrorKind::InvalidAccessToken => StatusCode::UNAUTHORIZED,
        }
    }
    fn message(&self) -> String {
        self.to_string()
    }
}
