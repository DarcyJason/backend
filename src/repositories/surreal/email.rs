use async_trait::async_trait;
use crate::custom::errors::AppError;
use crate::custom::errors::email::EmailErrorKind;
use crate::custom::result::AppResult;
use crate::database::surreal::client::SurrealClient;
use crate::models::email::{Email, TokenType};

#[async_trait]
pub trait EmailRepository {
    async fn create_email(&self, user_id: &str, token_type: TokenType, email_token: String) -> AppResult<()>;
}

#[async_trait]
impl EmailRepository for SurrealClient{
    async fn create_email(&self, user_id: &str, token_type: TokenType, email_token: String) -> AppResult<()> {
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
            .await?;
        let email: Option<Email> = result.take(0)?;
        match email {
            Some(_) => Ok(()),
            None => Err(AppError::EmailError(EmailErrorKind::CreateEmailFailed)),
        }
    }
}