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
    async fn create_refresh_token(&self, user_id: &str, token_value: &str) -> AppResult<RefreshToken>;
    async fn find_refresh_token_by_user_id(&self, user_id: &str)
    -> AppResult<Option<RefreshToken>>;
    async fn delete_refresh_token(&self, user_id: &str, token_value: &str) -> AppResult<()>;
}

#[async_trait]
impl RefreshTokenRepository for SurrealClient {
    async fn create_refresh_token(&self, user_id: &str, token_value: &str) -> AppResult<RefreshToken> {
        let sql = r#"
            CREATE refresh_tokens CONTENT {
                id: rand::uuid::v4(),
                user_id: $user_id,
                token_value: $token_value,
            }
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id.to_string()))
            .bind(("token_value", token_value.to_string()))
            .await?;
        let refresh_token: Option<RefreshToken> = result.take(0)?;
        match refresh_token {
            Some(refresh_token) => Ok(refresh_token),
            None => Err(AppError::RefreshTokenError(
                RefreshTokenErrorKind::CreateRefreshTokenFailed,
            )),
        }
    }
    async fn find_refresh_token_by_user_id(
        &self,
        user_id: &str,
    ) -> AppResult<Option<RefreshToken>> {
        let sql = r#"
            SELECT * FROM refresh_tokens WHERE user_id = <record> $user_id
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id.to_string()))
            .await?;
        let refresh_token: Option<RefreshToken> = result.take(0)?;
        Ok(refresh_token)
    }
    async fn delete_refresh_token(&self, user_id: &str, token_value: &str) -> AppResult<()> {
        let sql = r#"
            DELETE refresh_tokens WHERE user_id = <record> $user_id AND token_value = $token_value
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id.to_string()))
            .bind(("token_value", token_value.to_string()))
            .await?;
        let deleted_result: Option<RefreshToken> = result.take(0)?;
        match deleted_result {
            Some(_) => Ok(()),
            None => Err(AppError::RefreshTokenError(
                RefreshTokenErrorKind::DeleteRefreshTokenFailed,
            )),
        }
    }
}
