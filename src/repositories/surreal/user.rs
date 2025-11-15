use async_trait::async_trait;
use surrealdb::sql::Thing;

use crate::{
    core::{
        error::{external::ExternalError, user::UserErrorKind},
        result::AppResult,
    },
    database::surreal::client::SurrealClient,
    models::user::User,
    utils::password::hash_password,
};

#[async_trait]
pub trait UserRepository {
    async fn change_password(&self, user_id: Thing, new_password: &str) -> AppResult<()>;
}

#[async_trait]
impl UserRepository for SurrealClient {
    async fn change_password(&self, user_id: Thing, new_password: &str) -> AppResult<()> {
        let (new_password, new_salt) = hash_password(new_password.to_string())?;
        let sql = r#"
            UPDATE users SET password = $password,
            salt = $salt
            WHERE
                id = $user_id
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("password", new_password.to_string()))
            .bind(("salt", new_salt))
            .bind(("user_id", user_id))
            .await
            .map_err(ExternalError::from)?;
        let user: Option<User> = result.take(0).map_err(ExternalError::from)?;
        match user {
            Some(_) => Ok(()),
            None => Err(UserErrorKind::UserNotFound.into()),
        }
    }
}
