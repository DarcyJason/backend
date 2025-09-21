use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserErrorKind {
    #[error("User creation failed")]
    CreateFailed,
    #[error("User already exists")]
    AlreadyExists,
    #[error("User not found")]
    NotFound,
}
