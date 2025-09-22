use async_trait::async_trait;

use crate::{
    custom::{
        errors::{AppError, user::UserErrorKind},
        result::AppResult,
    },
    database::surreal::client::SurrealClient,
    models::user::User,
    utils::password::hash_password,
};
#[async_trait]
pub trait AuthRepository {
    async fn create_user(&self, name: String, email: String, password: String) -> AppResult<User>;
    async fn find_user_by_email(&self, email: String) -> AppResult<Option<User>>;
}

#[async_trait]
impl AuthRepository for SurrealClient {
    async fn create_user(&self, name: String, email: String, password: String) -> AppResult<User> {
        let (password, salt) = hash_password(password)?;
        let sql = r#"
            CREATE users CONTENT {
                id: rand::uuid::v4(),
                name: $name,
                email: $email,
                password: $password,
                role: $role,
                salt: $salt,
                is_active: $is_active
            }
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("name", name))
            .bind(("email", email))
            .bind(("password", password))
            .bind(("role", "user".to_string()))
            .bind(("salt", salt))
            .bind(("is_active", false))
            .await?;
        let user: Option<User> = result.take(0)?;
        user.ok_or_else(|| AppError::UserError(UserErrorKind::CreateUserFailed))
    }
    async fn find_user_by_email(&self, email: String) -> AppResult<Option<User>> {
        let sql = r#"
            SELECT * FROM users WHERE email = $email LIMIT 1
        "#;
        let mut result = self.client.query(sql).bind(("email", email)).await?;
        let user: Option<User> = result.take(0)?;
        Ok(user)
    }
}
