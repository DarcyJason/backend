use async_trait::async_trait;

use crate::{
    custom::{
        errors::{AppError, user::UserErrorKind},
        result::AppResult,
    },
    database::surreal::client::SurrealClient,
    models::user::User,
    security::crypto::password::hash_password,
};
#[async_trait]
pub trait AuthRepository {
    async fn create_user(
        &self,
        name: String,
        email: String,
        password: String,
        ip: String,
        device: String,
    ) -> AppResult<User>;
    async fn find_user_by_email(&self, email: String) -> AppResult<Option<User>>;
    async fn find_user_by_id(&self, id: String) -> AppResult<Option<User>>;
}

#[async_trait]
impl AuthRepository for SurrealClient {
    async fn create_user(
        &self,
        name: String,
        email: String,
        password: String,
        ip: String,
        device: String,
    ) -> AppResult<User> {
        let (password, salt) = hash_password(password)?;
        let sql = r#"
            CREATE users CONTENT {
                id: rand::uuid::v4(),
                name: $name,
                email: $email,
                password: $password,
                role: $role,
                salt: $salt,
                is_verified: false,
                ip: $ip,
                device: $device
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
            .bind(("ip", ip))
            .bind(("device", device))
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
    async fn find_user_by_id(&self, id: String) -> AppResult<Option<User>> {
        let sql = r#"
            SELECT * FROM users WHERE id = $id LIMIT 1
        "#;
        let mut result = self.client.query(sql).bind(("id", id)).await?;
        let user: Option<User> = result.take(0)?;
        Ok(user)
    }
}
