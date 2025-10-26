use crate::custom::errors::error_trait::ErrorKind;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExternalError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Redis(#[from] redis::RedisError),
    #[error(transparent)]
    SurrealDB(#[from] surrealdb::Error),
    #[error(transparent)]
    Figment(#[from] figment::Error),
    #[error(transparent)]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Resend(#[from] resend_rs::Error),
}

impl ErrorKind for ExternalError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    fn message(&self) -> String {
        // 生产环境下替换为下面的注释的内容，避免暴露内部细节
        // Replace the content of the following comments in the production environment to avoid exposing internal details.
        // "An internal server error occurred. Please try again later.".to_string()
        self.to_string()
    }
}
