use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};

use crate::{
    config::AppConfig,
    custom::{response::api::AppResponse, result::AppResult},
    database::client::DBClient,
    models::user::User,
    vo::user::MeVO,
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
        let me = MeVO::from(user);
        Ok(AppResponse::<MeVO>::success(
            StatusCode::OK.as_u16(),
            "OK",
            StatusCode::OK.canonical_reason().unwrap_or("OK"),
            Some(me),
        ))
    }
}
