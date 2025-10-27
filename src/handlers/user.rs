use std::sync::Arc;

use axum::{Extension, extract::State, response::IntoResponse};

use crate::{core::app_state::AppState, custom::result::AppResult, models::user::User};

pub async fn get_me(
    State(app_state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> AppResult<impl IntoResponse> {
    app_state.user_service.get_me(user).await
}

pub async fn update_user_profile() {}

pub async fn delete() {}

pub async fn change_password() {}

pub async fn refresh_token() {}
