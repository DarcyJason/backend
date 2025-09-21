use async_trait::async_trait;

use crate::{
    custom::{
        errors::{app_error::AppError, user::UserErrorKind},
        result::AppResult,
    },
    database::surreal::client::SurrealClient,
    models::user::User,
    utils::password::hash_password,
};
#[async_trait]
pub trait AuthRepository {
    async fn create_user(&self, name: String, email: String, password: String) -> AppResult<User>;
}

#[async_trait]
impl AuthRepository for SurrealClient {
    async fn create_user(&self, name: String, email: String, password: String) -> AppResult<User> {
        let (password, salt) = hash_password(password)?;
        let sql = r#"
            CREATE users SET
            name = $name,
            email = $email,
            password = $password,
            salt = $salt,
            is_active = $is_active
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("name", name))
            .bind(("email", email))
            .bind(("password", password))
            .bind(("salt", salt))
            .bind(("is_active", false))
            .await?;
        let created: Option<User> = result.take(0)?;
        created.ok_or_else(|| AppError::UserError(UserErrorKind::CreateFailed))
    }
}
