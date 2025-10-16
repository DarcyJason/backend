use crate::custom::errors::{AppError, validation::ValidationErrorKind};

impl From<figment::Error> for AppError {
    fn from(err: figment::Error) -> Self {
        AppError::AppConfigError(Box::new(err))
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::ValidationError(ValidationErrorKind::PasswordHashingError(err))
    }
}

impl From<surrealdb::Error> for AppError {
    fn from(err: surrealdb::Error) -> Self {
        AppError::SurrealDBError(Box::new(err))
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::RedisError(Box::new(err))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::OtherError(Box::new(err))
    }
}
