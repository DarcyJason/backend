use crate::custom::errors::email::EmailErrorKind;
use crate::custom::errors::external::ExternalError;
use crate::custom::result::AppResult;
use crate::database::surreal::client::SurrealClient;
use crate::models::email::{Email, TokenType};
use async_trait::async_trait;

#[async_trait]
pub trait EmailRepository {
    async fn create_email(
        &self,
        user_id: &str,
        token_type: TokenType,
        email_token: String,
    ) -> AppResult<()>;
    async fn find_verification_email_by_user_id(&self, user_id: &str) -> AppResult<Option<Email>>;
    async fn find_reset_password_email_by_user_id(&self, user_id: &str)
    -> AppResult<Option<Email>>;
}

#[async_trait]
impl EmailRepository for SurrealClient {
    async fn create_email(
        &self,
        user_id: &str,
        token_type: TokenType,
        email_token: String,
    ) -> AppResult<()> {
        let sql = r#"
            CREATE email CONTENT {
                id: rand::uuid::v4(),
                user_id: $user_id,
                token_type: $token_type,
                email_token: $email_token,
            }
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id.to_string()))
            .bind(("token_type", token_type))
            .bind(("email_token", email_token))
            .await
            .map_err(ExternalError::from)?;
        let email: Option<Email> = result.take(0).map_err(ExternalError::from)?;
        match email {
            Some(_) => Ok(()),
            None => Err(EmailErrorKind::CreateEmailFailed.into()),
        }
    }
    async fn find_verification_email_by_user_id(&self, user_id: &str) -> AppResult<Option<Email>> {
        let sql = r#"
            SELECT * FROM email WHERE user_id = $user_id AND token_type = 'Verification' ORDER BY created_at DESC LIMIT 1
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id.to_string()))
            .await
            .map_err(ExternalError::from)?;
        let email: Option<Email> = result.take(0).map_err(ExternalError::from)?;
        Ok(email)
    }
    async fn find_reset_password_email_by_user_id(
        &self,
        user_id: &str,
    ) -> AppResult<Option<Email>> {
        let sql = r#"
            SELECT * FROM email WHERE user_id = $user_id AND token_type = 'PasswordReset'
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id.to_string()))
            .await
            .map_err(ExternalError::from)?;
        let email: Option<Email> = result.take(0).map_err(ExternalError::from)?;
        Ok(email)
    }
}
