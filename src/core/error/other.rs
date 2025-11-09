use axum::http::StatusCode;
use thiserror::Error;

use crate::core::error::error_trait::ErrorKind;

#[derive(Debug, Error)]
pub enum OtherErrorKind {
    #[error("{0}")]
    Error(String),
}

impl ErrorKind for OtherErrorKind {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn message(&self) -> String {
        self.to_string()
    }
}
