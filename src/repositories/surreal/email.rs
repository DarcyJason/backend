use crate::core::error::email::EmailErrorKind;
use crate::core::error::external::ExternalError;
use crate::core::result::AppResult;
use crate::database::surreal::client::SurrealClient;
use crate::models::email::{Email, EmailType};
use async_trait::async_trait;
use surrealdb::sql::Thing;

#[async_trait]
pub trait EmailRepository {
    async fn create_email(
        &self,
        user_id: Thing,
        email_type: EmailType,
        email_token: String,
    ) -> AppResult<()>;
    async fn find_email_by_user_id_and_email_type(
        &self,
        user_id: Thing,
        email_type: EmailType,
    ) -> AppResult<Option<Email>>;
}

#[async_trait]
impl EmailRepository for SurrealClient {
    async fn create_email(
        &self,
        user_id: Thing,
        email_type: EmailType,
        email_token: String,
    ) -> AppResult<()> {
        let sql = r#"
            CREATE emails CONTENT {
                id: rand::uuid::v4(),
                user_id: $user_id,
                email_type: $email_type,
                email_token: $email_token,
            }
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id))
            .bind(("email_type", email_type))
            .bind(("email_token", email_token))
            .await
            .map_err(ExternalError::from)?;
        let email: Option<Email> = result.take(0).map_err(ExternalError::from)?;
        match email {
            Some(_) => Ok(()),
            None => Err(EmailErrorKind::CreateEmailFailed.into()),
        }
    }
    async fn find_email_by_user_id_and_email_type(
        &self,
        user_id: Thing,
        email_type: EmailType,
    ) -> AppResult<Option<Email>> {
        let sql = r#"
            SELECT * FROM emails
            WHERE
                user_id = $user_id AND
                email_type = $email_type AND
                is_used = false AND
                expires_at > time::now()
                ORDER BY created_at DESC
                LIMIT 1
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id))
            .bind(("email_type", email_type))
            .await
            .map_err(ExternalError::from)?;
        let email: Option<Email> = result.take(0).map_err(ExternalError::from)?;
        Ok(email)
    }
}
