use thiserror::Error;

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
    #[error("Failed to generate tokens")]
    TokenGenerationFailed,
}
