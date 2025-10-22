use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailErrorKind {
    #[error("Create email failed")]
    CreateEmailFailed,
    #[error("Email not found")]
    EmailNotFound,
}
