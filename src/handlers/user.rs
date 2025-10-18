use axum::{Extension, response::IntoResponse};

use crate::{
    custom::{response::AppResponse, result::AppResult},
    models::user::User,
};

pub async fn current_user(Extension(user): Extension<User>) -> AppResult<impl IntoResponse> {
    Ok(AppResponse::success(None, user))
}

pub async fn update_user_profile() {}

pub async fn delete_user() {}

pub async fn change_password() {}

pub async fn refresh_token() {}

pub async fn logout() {}
