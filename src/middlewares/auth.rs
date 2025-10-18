use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::IntoResponse,
};

use crate::{
    custom::{
        errors::{AppError, access_token::AccessTokenErrorKind, user::UserErrorKind},
        result::AppResult,
    },
    repositories::surreal::auth::AuthRepository,
    state::AppState,
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
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        }) {
        Some(token) => token,
        None => {
            return Err(AppError::AccessTokenError(
                AccessTokenErrorKind::AccessTokenNotFound,
            ));
        }
    };
    let user_id = match validate_access_token(
        access_token,
        app_state.config.jwt_config.jwt_secret.as_bytes(),
    ) {
        Ok(user_id) => user_id,
        Err(_) => {
            return Err(AppError::AccessTokenError(
                AccessTokenErrorKind::InvalidAccessToken,
            ));
        }
    };
    match app_state
        .db_client
        .surreal_client
        .find_user_by_id(&user_id)
        .await?
    {
        Some(user) => {
            req.extensions_mut().insert(user);
            Ok(next.run(req).await)
        }
        None => return Err(AppError::UserError(UserErrorKind::UserNotFound)),
    }
}
