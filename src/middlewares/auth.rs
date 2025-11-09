use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::IntoResponse,
};

use crate::{
    core::error::{access_token::AccessTokenErrorKind, user::UserErrorKind},
    core::{result::AppResult, state::AppState},
    models::user::{User, UserRole},
    repositories::surreal::auth::AuthRepository,
    utils::token::validate_access_token,
};

pub async fn auth(
    State(app_state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let access_token = match req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| auth_value.strip_prefix("Bearer ").map(|s| s.to_owned()))
    {
        Some(token) => token,
        None => {
            return Err(AccessTokenErrorKind::AccessTokenNotFound.into());
        }
    };
    let user_id = match validate_access_token(
        access_token,
        app_state.config.jwt_config.jwt_secret.as_bytes(),
    ) {
        Ok(user_id) => user_id,
        Err(_) => {
            return Err(AccessTokenErrorKind::InvalidAccessToken.into());
        }
    };
    match app_state
        .db_client
        .surreal_client
        .find_user_by_id(user_id)
        .await?
    {
        Some(user) => {
            req.extensions_mut().insert(user);
            Ok(next.run(req).await)
        }
        None => Err(UserErrorKind::UserNotFound.into()),
    }
}

pub async fn role_check(
    req: Request,
    next: Next,
    required_roles: Vec<UserRole>,
) -> AppResult<impl IntoResponse> {
    let user = req
        .extensions()
        .get::<User>()
        .ok_or(UserErrorKind::UserNotFound)?;
    if !required_roles.contains(&user.role) {
        return Err(UserErrorKind::Unauthorized.into());
    }
    Ok(next.run(req).await)
}
