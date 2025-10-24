use std::sync::Arc;

use axum::response::IntoResponse;
use tracing::{error, info};

use crate::{
    config::AppConfig,
    custom::{
        errors::{AppError, user::UserErrorKind},
        response::AppResponse,
        result::AppResult,
    },
    database::client::DBClient,
    dtos::requests::auth::RegisterRequest,
    repositories::surreal::auth::AuthRepository,
    validation::auth::validate_register_payload,
};

#[derive(Debug)]
pub struct AuthService {
    pub config: Arc<AppConfig>,
    pub db_client: Arc<DBClient>,
}

impl AuthService {
    pub fn new(config: Arc<AppConfig>, db_client: Arc<DBClient>) -> Self {
        Self { config, db_client }
    }
    pub async fn register(&self, payload: RegisterRequest) -> AppResult<impl IntoResponse + use<>> {
        validate_register_payload(&payload)?;
        if self
            .db_client
            .surreal_client
            .find_user_by_email(&payload.email)
            .await?
            .is_some()
        {
            error!(
                "❌ Failed: user already exists with email {}",
                payload.email
            );
            return Err(AppError::UserError(UserErrorKind::UserAlreadyExists));
        }
        match self
            .db_client
            .surreal_client
            .create_user(&payload.name, &payload.email, &payload.password)
            .await
        {
            Ok(_) => info!("Create user successfully"),
            Err(_) => {
                error!("Create user failed");
                return Err(AppError::UserError(UserErrorKind::CreateUserFailed));
            }
        }
        info!("✅ Start creating trusted device");
        info!(
            "✅ Finish Handling user registration successfully with email: {}",
            payload.email
        );
        Ok(AppResponse::success(
            Some("Register success".to_string()),
            None::<()>,
        ))
    }
}
