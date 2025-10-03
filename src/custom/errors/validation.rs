use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationErrorKind {
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    #[error("Password hashing error: {0}")]
    PasswordHashingError(#[from] argon2::password_hash::Error),
}
