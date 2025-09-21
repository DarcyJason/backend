use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationErrorKind {
    #[error("Name is too long, it should be at most 20 characters")]
    NameTooLong,
    #[error("Email is invalid")]
    InvalidEmail,
    #[error("Password is too short, minimum length is 8 characters")]
    PasswordTooShort,
    #[error("Password is too long, maximum length is 20 characters")]
    PasswordTooLong,
    #[error("Confirm password is too short, minimum length is 8 characters")]
    ConfirmPasswordTooShort,
    #[error("Confirm password is too long, maximum length is 20 characters")]
    ConfirmPasswordTooLong,
    #[error("Password and confirm password are not match")]
    PasswordAndConfirmPasswordAreNotMatch,
    #[error("Password hashing error: {0}")]
    PasswordHashingError(#[from] argon2::password_hash::Error),
    #[error("Field '{0}' cannot be empty")]
    EmptyField(String),
}
