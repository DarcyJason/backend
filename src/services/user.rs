use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};

use crate::{
    config::AppConfig,
    custom::{response::AppResponse, result::AppResult},
    database::client::DBClient,
    dtos::responses::user::MeResponseData,
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
        let me = MeResponseData::from(user);
        Ok(AppResponse::<MeResponseData>::success(
            StatusCode::OK.as_u16(),
            "OK",
            StatusCode::OK.canonical_reason().unwrap_or("OK"),
            Some(me),
        ))
    }
}
