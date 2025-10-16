use crate::{
    custom::{
        errors::{AppError, refresh_token::RefreshTokenErrorKind},
        result::AppResult,
    },
    database::surreal::client::SurrealClient,
    models::token::RefreshToken,
};
use async_trait::async_trait;

#[async_trait]
pub trait RefreshTokenRepository {
    async fn create_refresh_token(&self, user_id: &str, refresh_token: &str) -> AppResult<()>;
}

#[async_trait]
impl RefreshTokenRepository for SurrealClient {
    async fn create_refresh_token(&self, user_id: &str, refresh_token: &str) -> AppResult<()> {
        let sql = r#"
            CREATE refresh_token CONTENT {
                id: rand::uuid::v4(),
                user_id: $user_id,
                token: $refresh_token,
                revoke: false,
            }
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id.to_string()))
            .bind(("refresh_token", refresh_token.to_string()))
            .await?;
        let refresh_token: Option<RefreshToken> = result.take(0)?;
        match refresh_token {
            Some(_) => Ok(()),
            None => Err(AppError::RefreshTokenError(
                RefreshTokenErrorKind::CreateRefreshTokenFailed,
            )),
        }
    }
}
