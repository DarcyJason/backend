use crate::{
    core::result::AppResult,
    core::errors::{external::ExternalError, refresh_token::RefreshTokenErrorKind},
    database::surreal::client::SurrealClient,
    models::token::RefreshToken,
};
use async_trait::async_trait;
use surrealdb::sql::Thing;

#[async_trait]
pub trait RefreshTokenRepository {
    async fn create_refresh_token(
        &self,
        user_id: Thing,
        device_id: Thing,
        token_value: &str,
    ) -> AppResult<RefreshToken>;
    async fn find_refresh_token_by_user_and_device(
        &self,
        user_id: Thing,
        device_id: Thing,
    ) -> AppResult<Option<RefreshToken>>;
    async fn delete_refresh_token(&self, user_id: Thing, token_value: &str) -> AppResult<()>;
}

#[async_trait]
impl RefreshTokenRepository for SurrealClient {
    async fn create_refresh_token(
        &self,
        user_id: Thing,
        device_id: Thing,
        token_value: &str,
    ) -> AppResult<RefreshToken> {
        let sql = r#"
            CREATE refresh_tokens CONTENT {
                id: rand::uuid::v4(),
                user_id: $user_id,
                device_id: $device_id,
                token_value: $token_value,
            }
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id))
            .bind(("device_id", device_id))
            .bind(("token_value", token_value.to_string()))
            .await
            .map_err(ExternalError::from)?;
        let mut refresh_token: Vec<RefreshToken> = result.take(0).map_err(ExternalError::from)?;
        match refresh_token.pop() {
            Some(refresh_token) => Ok(refresh_token),
            None => Err(RefreshTokenErrorKind::CreateRefreshTokenFailed.into()),
        }
    }
    async fn find_refresh_token_by_user_and_device(
        &self,
        user_id: Thing,
        device_id: Thing,
    ) -> AppResult<Option<RefreshToken>> {
        let sql = r#"
            SELECT * FROM refresh_tokens WHERE user_id = $user_id AND device_id = $device_id
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id))
            .bind(("device_id", device_id))
            .await
            .map_err(ExternalError::from)?;
        let mut refresh_token: Vec<RefreshToken> = result.take(0).map_err(ExternalError::from)?;
        Ok(refresh_token.pop())
    }
    async fn delete_refresh_token(&self, user_id: Thing, token_value: &str) -> AppResult<()> {
        let sql = r#"
            DELETE * FROM refresh_tokens WHERE user_id = $user_id AND token_value = $token_value
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id))
            .bind(("token_value", token_value.to_string()))
            .await
            .map_err(ExternalError::from)?;
        let deleted_result: Vec<RefreshToken> = result.take(0).map_err(ExternalError::from)?;
        if deleted_result.is_empty() {
            return Err(RefreshTokenErrorKind::DeleteRefreshTokenFailed.into());
        }
        Ok(())
    }
}
