use crate::{
    core::{error::external::ExternalError, result::AppResult},
    database::redis::client::RedisClient,
    models::{email::EmailType, user::User},
};
use async_trait::async_trait;
use redis::AsyncTypedCommands;
use surrealdb::sql::Thing;

#[async_trait]
pub trait AuthCacheRepository {
    async fn set_user(&self, user: &User, ttl_seconds: u64) -> AppResult<()>;
    async fn get_user(&self, user_id: &Thing) -> AppResult<Option<User>>;
    async fn delete_user(&self, user_id: &Thing) -> AppResult<()>;
    async fn set_email_token(
        &self,
        email_token_type: EmailType,
        email_token: &str,
        user_id: &Thing,
        ttl_seconds: u64,
    ) -> AppResult<()>;
    async fn use_email_token(
        &self,
        email_token_type: EmailType,
        email_token: &str,
    ) -> AppResult<Option<Thing>>;
    async fn add_jti_to_blacklist(&self, jti: &str, ttl_seconds: u64) -> AppResult<()>;
    async fn is_jti_in_blacklist(&self, jti: &str) -> AppResult<bool>;
}

#[async_trait]
impl AuthCacheRepository for RedisClient {
    async fn set_user(&self, user: &User, ttl_seconds: u64) -> AppResult<()> {
        let key = format!("user:{}", user.id);
        let user_json = serde_json::to_string(user).map_err(ExternalError::from)?;
        let mut conn = self.conn.clone();
        conn.set_ex(key, user_json, ttl_seconds)
            .await
            .map_err(ExternalError::from)?;
        Ok(())
    }
    async fn get_user(&self, user_id: &Thing) -> AppResult<Option<User>> {
        let key = format!("user:{}", user_id);
        let mut conn = self.conn.clone();
        let user_json: Option<String> = conn.get(key).await.map_err(ExternalError::from)?;
        match user_json {
            Some(json) => {
                let user: User = serde_json::from_str(&json).map_err(ExternalError::from)?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
    async fn delete_user(&self, user_id: &Thing) -> AppResult<()> {
        let key = format!("user:{}", user_id);
        let mut conn = self.conn.clone();
        conn.del(key).await.map_err(ExternalError::from)?;
        Ok(())
    }
    async fn set_email_token(
        &self,
        email_token_type: EmailType,
        email_token: &str,
        user_id: &Thing,
        ttl_seconds: u64,
    ) -> AppResult<()> {
        let key = format!("temp_token:{}:{}", email_token_type, email_token);
        let mut conn = self.conn.clone();
        conn.set_ex(key, user_id.to_string(), ttl_seconds)
            .await
            .map_err(ExternalError::from)?;
        Ok(())
    }
    async fn use_email_token(
        &self,
        email_token_type: EmailType,
        email_token: &str,
    ) -> AppResult<Option<Thing>> {
        let key = format!("temp_token:{}:{}", email_token_type, email_token);
        let mut conn = self.conn.clone();
        let user_id_str: Option<String> = conn.get_del(key).await.map_err(ExternalError::from)?;
        Ok(user_id_str.and_then(|s| s.parse::<Thing>().ok()))
    }
    async fn add_jti_to_blacklist(&self, jti: &str, ttl_seconds: u64) -> AppResult<()> {
        let key = format!("blacklist:jti:{}", jti);
        let mut conn = self.conn.clone();
        conn.set_ex(key, "1", ttl_seconds)
            .await
            .map_err(ExternalError::from)?;
        Ok(())
    }
    async fn is_jti_in_blacklist(&self, jti: &str) -> AppResult<bool> {
        let key = format!("blacklist:jti:{}", jti);
        let mut conn = self.conn.clone();
        let exists: bool = conn.exists(key).await.map_err(ExternalError::from)?;
        Ok(exists)
    }
}
