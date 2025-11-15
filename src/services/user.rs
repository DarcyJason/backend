use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};

use crate::{
    core::{
        config::AppConfig, error::user::UserErrorKind, response::AppResponse, result::AppResult,
    },
    database::client::DBClient,
    dto::{request::user::ChangePasswordRequest, response::user::MeResponse},
    models::user::User,
    repositories::{
        redis::auth::AuthCacheRepository,
        surreal::{auth::AuthRepository, user::UserRepository},
    },
    utils::password::compare_hashed_password,
    validation::user::validate_change_password_request,
};

#[derive(Debug)]
pub struct UserService {
    pub config: Arc<AppConfig>,
    pub db_client: Arc<DBClient>,
}

impl UserService {
    pub fn new(config: Arc<AppConfig>, db_client: Arc<DBClient>) -> Self {
        Self { config, db_client }
    }
    pub async fn get_me(&self, user: User) -> AppResult<impl IntoResponse + use<>> {
        let me = MeResponse::from(user);
        Ok(AppResponse::<MeResponse>::success(
            StatusCode::OK.as_u16(),
            "OK",
            StatusCode::OK.canonical_reason().unwrap_or("OK"),
            Some(me),
        ))
    }
    pub async fn change_password(
        &self,
        user: User,
        payload: ChangePasswordRequest,
    ) -> AppResult<impl IntoResponse + use<>> {
        validate_change_password_request(&payload)?;
        let user_detail = match self
            .db_client
            .surreal_client
            .find_user_by_id(user.id.clone())
            .await?
        {
            Some(user) => user,
            None => return Err(UserErrorKind::UserNotFound.into()),
        };
        if compare_hashed_password(&payload.new_password, &user_detail.password)? {
            return Err(UserErrorKind::PasswordMustBeDifferentFromLastPassword.into());
        }
        self.db_client
            .surreal_client
            .change_password(user.id.clone(), &payload.new_password)
            .await?;
        self.db_client.redis_client.delete_user(&user.id).await?;
        Ok(AppResponse::<()>::success(
            StatusCode::OK.as_u16(),
            "Reset your password successfully",
            StatusCode::OK.canonical_reason().unwrap_or("OK"),
            None,
        ))
    }
}
