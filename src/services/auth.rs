use resend_rs::Resend;
use std::{net::SocketAddr, sync::Arc};

use axum::{
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    response::IntoResponse,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use time::Duration;
use tracing::{error, info};

use crate::{
    core::{
        config::AppConfig,
        error::{email::EmailErrorKind, external::ExternalError, user::UserErrorKind},
        response::AppResponse,
        result::AppResult,
    },
    database::client::DBClient,
    dto::{
        request::auth::{
            ForgetPasswordRequest, LoginRequest, RegisterRequest, ResetPasswordRequest,
            VerifyUserRequest,
        },
        response::auth::{LoginResponse, VerifyUserResponse},
    },
    models::{
        email::EmailType,
        user::{User, UserStatus},
    },
    repositories::{
        redis::auth::AuthCacheRepository,
        surreal::{
            auth::AuthRepository, device::DeviceRepository, refresh_token::RefreshTokenRepository,
        },
    },
    templates::{
        reset_password_email_html::RESET_PASSWORD_EMAIL_HTML,
        verification_email_html::VERIFICATION_EMAIL_HTML,
    },
    utils::{
        device::parse_user_agent_detailed,
        mail::send_mail,
        password::compare_hashed_password,
        token::{generate_access_token, generate_email_token, generate_refresh_token},
    },
    validation::auth::{
        validate_forget_password_payload, validate_login_payload, validate_register_payload,
        validate_reset_password_payload, validate_verify_user_payload,
    },
};

#[derive(Debug)]
pub struct AuthService {
    pub config: Arc<AppConfig>,
    pub db_client: Arc<DBClient>,
    pub resend: Arc<Resend>,
}

impl AuthService {
    pub fn new(config: Arc<AppConfig>, db_client: Arc<DBClient>, resend: Arc<Resend>) -> Self {
        Self {
            config,
            db_client,
            resend,
        }
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
            return Err(UserErrorKind::UserAlreadyExists.into());
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
                return Err(UserErrorKind::CreateUserFailed.into());
            }
        }
        Ok(AppResponse::<()>::success(
            StatusCode::OK.as_u16(),
            "register success",
            StatusCode::OK.canonical_reason().unwrap_or("OK"),
            None,
        ))
    }
    pub async fn login(
        &self,
        headers: HeaderMap,
        jar: CookieJar,
        payload: LoginRequest,
    ) -> AppResult<impl IntoResponse + use<>> {
        validate_login_payload(&payload)?;
        let user = match self
            .db_client
            .surreal_client
            .find_user_by_email(&payload.email)
            .await?
        {
            Some(user) => user,
            None => return Err(UserErrorKind::UserNotFound.into()),
        };
        if !compare_hashed_password(&payload.password, &user.password)? {
            return Err(UserErrorKind::WrongPassword.into());
        }
        if !user.is_verified {
            let email_token = generate_email_token();
            // Use Redis to store verification token (e.g., for 30 minutes)
            self.db_client
                .redis_client
                .set_email_token(EmailType::Verification, &email_token, &user.id, 1800)
                .await?;

            let html = VERIFICATION_EMAIL_HTML
                .replace("{{username}}", &user.name)
                .replace("{{email_token}}", &email_token);
            let _email = send_mail(
                &self.resend,
                &self.config.mail_server.from_email,
                vec![&user.email],
                "Verification",
                &html,
            )
            .await
            .map_err(ExternalError::from)?;
            let response_headers = HeaderMap::new();
            return Ok((
                response_headers,
                jar,
                AppResponse::<LoginResponse>::success(
                    StatusCode::OK.as_u16(),
                    "Check your email",
                    StatusCode::OK.canonical_reason().unwrap_or("OK"),
                    Some(LoginResponse {
                        device: None,
                        need_verification: true,
                    }),
                ),
            ));
        }
        let user_agent_str = match headers.get("User-Agent").and_then(|ua| ua.to_str().ok()) {
            Some(user_agent) => user_agent,
            None => return Err(UserErrorKind::MissingUserAgent.into()),
        };
        let (user_agent, os, device) = parse_user_agent_detailed(user_agent_str);
        let trusted_devices = self
            .db_client
            .surreal_client
            .find_trusted_devices_by_user_id(user.id.clone())
            .await?;
        let found_device = trusted_devices
            .into_iter()
            .find(|d| d.user_agent == user_agent && d.os == os && d.device == device);
        let device = match found_device {
            Some(device) => device,
            None => {
                let email_token = generate_email_token();
                // Use Redis to store verification token
                self.db_client
                    .redis_client
                    .set_email_token(EmailType::Verification, &email_token, &user.id, 1800)
                    .await?;

                let html = VERIFICATION_EMAIL_HTML
                    .replace("{{username}}", &user.name)
                    .replace("{{email_token}}", &email_token);
                let _email = send_mail(
                    &self.resend,
                    &self.config.mail_server.from_email,
                    vec![&user.email],
                    "Verification",
                    &html,
                )
                .await
                .map_err(ExternalError::from)?;
                let response_headers = HeaderMap::new();
                return Ok((
                    response_headers,
                    jar,
                    AppResponse::<LoginResponse>::success(
                        StatusCode::OK.as_u16(),
                        "Check your email",
                        StatusCode::OK.canonical_reason().unwrap_or("OK"),
                        Some(LoginResponse {
                            device: None,
                            need_verification: true,
                        }),
                    ),
                ));
            }
        };
        let refresh_token_value = match self
            .db_client
            .surreal_client
            .find_refresh_token_by_user_and_device(user.id.clone(), device.id.clone())
            .await?
        {
            Some(token) => token.token_value,
            None => {
                let new_token_value = generate_refresh_token();
                self.db_client
                    .surreal_client
                    .create_refresh_token(user.id.clone(), device.id.clone(), &new_token_value)
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
        let jar = jar.add(refresh_token_cookie);
        let access_token = generate_access_token(
            user.id.clone(),
            self.config.jwt_config.jwt_secret.as_bytes(),
            self.config.jwt_config.jwt_expires_in_seconds,
        )?;
        let mut response_headers = HeaderMap::new();
        response_headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", access_token).parse().unwrap(),
        );
        Ok((
            response_headers,
            jar,
            AppResponse::<LoginResponse>::success(
                StatusCode::OK.as_u16(),
                &format!("Login successfully, {}", user.email),
                StatusCode::OK.canonical_reason().unwrap_or("OK"),
                Some(LoginResponse {
                    device: Some(device),
                    need_verification: false,
                }),
            ),
        ))
    }
    pub async fn logout(&self, jar: CookieJar, user: User) -> AppResult<impl IntoResponse + use<>> {
        if let Some(cookie) = jar.get("refresh_token") {
            let refresh_token = cookie.value().to_string();
            match self
                .db_client
                .surreal_client
                .delete_refresh_token(user.id.clone(), &refresh_token)
                .await
            {
                Ok(_) => info!("✅ Successfully deleted refresh token from database."),
                Err(e) => error!(
                    "❌ Failed to delete refresh_token from DB for user_id {}: {}. Proceeding to clear cookie.",
                    &user.id, e
                ),
            }
        }
        let new_refresh_token_cookie = Cookie::build(("refresh_token", ""))
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .max_age(Duration::ZERO)
            .build();
        let updated_jar = jar.remove("refresh_token").add(new_refresh_token_cookie);
        Ok((
            updated_jar,
            AppResponse::<()>::success(
                StatusCode::OK.as_u16(),
                "Logout Success",
                StatusCode::OK.canonical_reason().unwrap_or("OK"),
                None,
            ),
        ))
    }
    pub async fn verify_email(
        &self,
        headers: HeaderMap,
        addr: SocketAddr,
        payload: VerifyUserRequest,
    ) -> AppResult<impl IntoResponse + use<>> {
        validate_verify_user_payload(&payload)?;
        let user_id = match self
            .db_client
            .redis_client
            .use_email_token(EmailType::Verification, &payload.email_token)
            .await?
        {
            Some(user_id) => user_id,
            None => return Err(EmailErrorKind::InvalidToken.into()),
        };
        let user = self
            .db_client
            .surreal_client
            .find_user_by_id(user_id.clone())
            .await?
            .ok_or(UserErrorKind::UserNotFound)?;

        if user.email != payload.email {
            return Err(UserErrorKind::Unauthorized.into());
        }
        self.db_client
            .surreal_client
            .user_verified(user.id.clone(), UserStatus::Active)
            .await?;
        self.db_client.redis_client.delete_user(&user.id).await?;
        let user_agent_str = match headers.get("User-Agent").and_then(|ua| ua.to_str().ok()) {
            Some(user_agent) => user_agent,
            None => return Err(UserErrorKind::MissingUserAgent.into()),
        };
        let (user_agent, os, device) = parse_user_agent_detailed(user_agent_str);
        let new_device = self
            .db_client
            .surreal_client
            .create_device(
                user.id.clone(),
                user_agent,
                os,
                device,
                addr.ip().to_string(),
            )
            .await?;
        Ok(AppResponse::<VerifyUserResponse>::success(
            StatusCode::OK.as_u16(),
            "Verify your account successfully",
            StatusCode::OK.canonical_reason().unwrap_or("OK"),
            Some(VerifyUserResponse { device: new_device }),
        ))
    }
    pub async fn forget_password(
        &self,
        payload: ForgetPasswordRequest,
    ) -> AppResult<impl IntoResponse + use<>> {
        validate_forget_password_payload(&payload)?;
        let user = match self
            .db_client
            .surreal_client
            .find_user_by_email(&payload.email)
            .await?
        {
            Some(user) => user,
            None => return Err(UserErrorKind::UserNotFound.into()),
        };
        let email_token = generate_email_token();
        self.db_client
            .redis_client
            .set_email_token(EmailType::PasswordReset, &email_token, &user.id, 1800)
            .await?;
        let html = RESET_PASSWORD_EMAIL_HTML
            .replace("{{username}}", &user.name)
            .replace("{{email_token}}", &email_token);
        let _email = send_mail(
            &self.resend,
            &self.config.mail_server.from_email,
            vec![&user.email],
            "Reset password",
            &html,
        )
        .await
        .map_err(ExternalError::from)?;
        Ok(AppResponse::<()>::success(
            StatusCode::OK.as_u16(),
            "An reset password email has been sent, please check your email",
            StatusCode::OK.canonical_reason().unwrap_or("OK"),
            None,
        ))
    }
    pub async fn reset_password(
        &self,
        payload: ResetPasswordRequest,
    ) -> AppResult<impl IntoResponse + use<>> {
        validate_reset_password_payload(&payload)?;
        let user_id = match self
            .db_client
            .redis_client
            .use_email_token(EmailType::PasswordReset, &payload.token)
            .await?
        {
            Some(user_id) => user_id,
            None => return Err(EmailErrorKind::InvalidToken.into()),
        };
        let user = self
            .db_client
            .surreal_client
            .find_user_by_id(user_id.clone())
            .await?
            .ok_or(UserErrorKind::UserNotFound)?;

        if user.email != payload.email {
            return Err(UserErrorKind::Unauthorized.into());
        }
        self.db_client
            .surreal_client
            .reset_password(user.id.clone(), &payload.new_password)
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
