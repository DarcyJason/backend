use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccessTokenErrorKind {
    #[error("Access token not found")]
    AccessTokenNotFound,
    #[error("Invalid access token")]
    InvalidAccessToken,
}
