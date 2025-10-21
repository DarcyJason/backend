use axum::{
    Extension, Json,
    extract::{ConnectInfo, OriginalUri, State},
    http::{HeaderMap, header::AUTHORIZATION},
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use std::{net::SocketAddr, sync::Arc};
use time::Duration;
use tracing::{error, info, instrument};

use crate::utils::token::generate_email_token;
use crate::{
    custom::{
        errors::{AppError, user::UserErrorKind},
        response::AppResponse,
        result::AppResult,
    },
    dtos::{
        requests::{login::LoginRequest, register::RegisterRequest},
        responses::login::LoginResponseData,
    },
    models::user::User,
    repositories::surreal::{
        auth::AuthRepository, device::DeviceRepository, refresh_token::RefreshTokenRepository,
    },
    state::AppState,
    utils::{
        device::parse_user_agent_detailed,
        password::compare_hashed_password,
        token::{generate_access_token, generate_refresh_token},
    },
};
use crate::{models::email::TokenType, validation::auth::validate_login_payload};
use crate::{
    repositories::surreal::email::EmailRepository, validation::auth::validate_register_payload,
};

#[instrument(skip(app_state))]
pub async fn register(
    State(app_state): State<Arc<AppState>>,
    uri: OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    info!("✅ Start handling user registration");
    validate_register_payload(&payload)?;
    if app_state
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
    match app_state
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
        (),
    ))
}

#[instrument(skip(app_state, headers))]
pub async fn login(
    State(app_state): State<Arc<AppState>>,
    jar: CookieJar,
    uri: OriginalUri,
    mut headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    info!("✅ Start handling user login");
    validate_login_payload(&payload)?;
    let user = match app_state
        .db_client
        .surreal_client
        .find_user_by_email(&payload.email)
        .await?
    {
        Some(user) => user,
        None => return Err(AppError::UserError(UserErrorKind::UserNotFound)),
    };
    if !compare_hashed_password(&payload.password, &user.password)? {
        return Err(AppError::UserError(UserErrorKind::WrongPassword));
    }
    info!("✅ Start getting user device");
    let user_agent_str = match headers.get("user-agent").and_then(|ua| ua.to_str().ok()) {
        Some(user_agent) => user_agent,
        None => return Err(AppError::UserError(UserErrorKind::MissingUserAgent)),
    };
    let (user_agent, os, device) = parse_user_agent_detailed(user_agent_str);
    let trusted_devices = app_state
        .db_client
        .surreal_client
        .find_trusted_devices_by_user_id(&user.id.to_string())
        .await?;
    let found_device = trusted_devices
        .iter()
        .find(|d| d.user_agent == user_agent && d.os == os && d.device == device);
    let (device_id, response_message, is_new_device) = if let Some(trusted_device) = found_device {
        let message = if user.is_verified {
            format!("Login success with trusted device for user {}", &user.email)
        } else {
            let email_token = generate_email_token();
            app_state
                .db_client
                .surreal_client
                .create_email(&user.id.to_string(), TokenType::Verification, email_token)
                .await?;
            "Your device is recognized, but your account is not verified. A new verification email has been sent.".to_string()
        };
        (trusted_device.id.to_string(), message, false)
    } else {
        let new_device = app_state
            .db_client
            .surreal_client
            .create_device(
                &user.id.to_string(),
                user_agent,
                os,
                device,
                addr.ip().to_string(),
            )
            .await?;
        let email_token = generate_email_token();
        app_state
            .db_client
            .surreal_client
            .create_email(&user.id.to_string(), TokenType::Verification, email_token)
            .await?;
        (
            new_device.id.to_string(),
            "This is a new device. A verification email has been sent to you, please check it."
                .to_string(),
            true,
        )
    };
    let response_device = if is_new_device {
        app_state
            .db_client
            .surreal_client
            .find_device_by_id(&device_id)
            .await?
            .unwrap()
    } else {
        found_device.unwrap().clone()
    };
    info!("✅ Start creating access_token");
    if headers
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .is_none()
    {
        let access_token = generate_access_token(
            user.id.to_string(),
            app_state.config.jwt_config.jwt_secret.as_bytes(),
            app_state.config.jwt_config.jwt_expires_in_seconds,
        )?;
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", access_token).parse().unwrap(),
        );
    }
    info!("✅ Start creating refresh token");
    let jar = if jar.get("refresh_token").is_none() {
        let refresh_token_value = match app_state
            .db_client
            .surreal_client
            .find_refresh_token_by_user_and_device(&user.id.to_string(), &device_id)
            .await?
        {
            Some(token) => token.token_value,
            None => {
                let new_token_value = generate_refresh_token();
                app_state
                    .db_client
                    .surreal_client
                    .create_refresh_token(&user.id.to_string(), &device_id, &new_token_value)
                    .await?;
                new_token_value
            }
        };
        let refresh_token_cookie = Cookie::build(("refresh_token", refresh_token_value))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .max_age(Duration::days(7))
            .build();
        jar.add(refresh_token_cookie)
    } else {
        jar
    };
    info!("✅ Login process completed for user {}", &user.email);
    Ok((
        headers,
        jar,
        AppResponse::success(
            Some(response_message),
            LoginResponseData {
                device: response_device,
            },
        ),
    ))
}

#[instrument(skip(app_state, jar, user))]
pub async fn logout(
    State(app_state): State<Arc<AppState>>,
    jar: CookieJar,
    Extension(user): Extension<User>,
) -> AppResult<impl IntoResponse> {
    info!("✅ Start handling user logout for user_id: {}", user.id);
    if let Some(cookie) = jar.get("refresh_token") {
        let refresh_token = cookie.value().to_string();
        // Attempt to delete the token from the database, but don't let failure block logout.
        match app_state
            .db_client
            .surreal_client
            .delete_refresh_token(&user.id.to_string(), &refresh_token)
            .await
        {
            Ok(_) => info!("✅ Successfully deleted refresh token from database."),
            Err(e) => error!(
                "❌ Failed to delete refresh_token from DB for user_id {}: {}. Proceeding to clear cookie.",
                &user.id, e
            ),
        }
    }
    let refresh_token_cookie = Cookie::build(("refresh_token", ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(Duration::ZERO)
        .build();
    let updated_jar = jar.remove("refresh_token").add(refresh_token_cookie);
    info!("✅ Logout success for user_id: {}", user.id);
    Ok((
        updated_jar,
        AppResponse::success(Some("Logout Success".to_string()), ()),
    ))
}

#[instrument(skip(_app_state))]
pub async fn forget_password(State(_app_state): State<Arc<AppState>>) {}

#[instrument]
pub async fn reset_password() {}
