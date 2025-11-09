use axum::http::StatusCode;
use thiserror::Error;

use crate::core::error::error_trait::ErrorKind;

#[derive(Debug, Error)]
pub enum DeviceErrorKind {
    #[error("Create trust device failed")]
    CreateDeviceFailed,
    #[error("Trusted device not found")]
    DeviceNotFound,
}

impl ErrorKind for DeviceErrorKind {
    fn status_code(&self) -> StatusCode {
        match self {
            DeviceErrorKind::CreateDeviceFailed => StatusCode::INTERNAL_SERVER_ERROR,
            DeviceErrorKind::DeviceNotFound => StatusCode::NOT_FOUND,
        }
    }
    fn message(&self) -> String {
        self.to_string()
    }
}
