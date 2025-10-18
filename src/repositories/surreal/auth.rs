use async_trait::async_trait;

use crate::{
    custom::{
        errors::{AppError, user::UserErrorKind},
        result::AppResult,
    },
    database::surreal::client::SurrealClient,
    models::user::{User, UserRole, UserStatus},
    utils::password::hash_password,
};

#[async_trait]
pub trait AuthRepository {
    async fn create_user(&self, name: &str, email: &str, password: &str) -> AppResult<()>;
    async fn find_user_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn find_user_by_id(&self, id: &str) -> AppResult<Option<User>>;
}

#[async_trait]
impl AuthRepository for SurrealClient {
    async fn create_user(&self, name: &str, email: &str, password: &str) -> AppResult<()> {
        let (password, salt) = hash_password(password.to_string())?;
        let sql = r#"
            CREATE users CONTENT {
                id: rand::uuid::v4(),
                name: $name,
                email: $email,
                password: $password,
                role: $role,
                salt: $salt,
                is_verified: false,
                status: $status,
            }
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("name", name.to_string()))
            .bind(("email", email.to_string()))
            .bind(("password", password))
            .bind(("role", UserRole::User))
            .bind(("salt", salt))
            .bind(("status", UserStatus::Inactive))
            .await?;
        let user: Option<User> = result.take(0)?;
        match user {
            Some(_) => Ok(()),
            None => Err(AppError::UserError(UserErrorKind::CreateUserFailed)),
        }
    }
    async fn find_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let sql = r#"
            SELECT * FROM users WHERE email = $email LIMIT 1
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("email", email.to_string()))
            .await?;
        let user: Option<User> = result.take(0)?;
        Ok(user)
    }
    async fn find_user_by_id(&self, id: &str) -> AppResult<Option<User>> {
        let sql = r#"
            SELECT * FROM users WHERE id = <record> $id LIMIT 1
        "#;
        let mut result = self.client.query(sql).bind(("id", id.to_string())).await?;
        let user: Option<User> = result.take(0)?;
        Ok(user)
    }
}
