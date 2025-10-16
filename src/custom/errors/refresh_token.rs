use thiserror::Error;

#[derive(Debug, Error)]
pub enum RefreshTokenErrorKind {
    #[error("Create refresh token failed")]
    CreateRefreshTokenFailed,
}
