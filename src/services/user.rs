use std::sync::Arc;

use axum::response::IntoResponse;

use crate::{
    config::AppConfig,
    custom::{response::AppResponse, result::AppResult},
    database::client::DBClient,
    models::user::User,
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
        Ok(AppResponse::success(None, user))
    }
}
