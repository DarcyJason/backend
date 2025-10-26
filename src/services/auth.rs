use std::{net::SocketAddr, sync::Arc};

use axum::{
    http::{HeaderMap, header::AUTHORIZATION},
    response::IntoResponse,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use time::Duration;
use tracing::{error, info};

use crate::{
    config::AppConfig,
    custom::{
        errors::{email::EmailErrorKind, external::ExternalError, user::UserErrorKind},
        response::AppResponse,
        result::AppResult,
    },
    database::client::DBClient,
    dtos::{
        requests::auth::{
            ForgetPasswordRequest, LoginRequest, RegisterRequest, ResetPasswordRequest,
            VerifyUserRequest,
        },
        responses::login::LoginResponseData,
    },
    mail::{send_mail::send_mail, templates::verification_email_html::VERIFICATION_EMAIL_HTML},
    models::{email::TokenType, user::User},
    repositories::surreal::{
        auth::AuthRepository, device::DeviceRepository, email::EmailRepository,
        refresh_token::RefreshTokenRepository,
    },
    utils::{
        device::parse_user_agent_detailed,
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
            info!("✅ Start letting new user to verify account");
            let email_token = generate_email_token();
            self.db_client
                .surreal_client
                .create_email(
                    &user.id.to_string(),
                    TokenType::Verification,
                    email_token.clone(),
                )
                .await?;
            let html = VERIFICATION_EMAIL_HTML
                .replace("{{username}}", &user.name)
                .replace("{{email_token}}", &email_token);
            let _email = send_mail(
                &self.config.mail_server.from_email,
                vec![&user.email],
                "Verification",
                &html,
                &self.config.mail_server.resend_api_key,
            )
            .await
            .map_err(ExternalError::from)?;
            let response_headers = HeaderMap::new();
            return Ok((
                response_headers,
                jar,
                AppResponse::success(Some("Check your email".to_string()), None),
            ));
        }
        info!("✅ Start getting user device");
        let user_agent_str = match headers.get("User-Agent").and_then(|ua| ua.to_str().ok()) {
            Some(user_agent) => user_agent,
            None => return Err(UserErrorKind::MissingUserAgent.into()),
        };
        let (user_agent, os, device) = parse_user_agent_detailed(user_agent_str);
        let trusted_devices = self
            .db_client
            .surreal_client
            .find_trusted_devices_by_user_id(&user.id.to_string())
            .await?;
        let found_device = trusted_devices
            .into_iter()
            .find(|d| d.user_agent == user_agent && d.os == os && d.device == device);
        let device = match found_device {
            Some(device) => device,
            None => {
                info!("✅ Start letting user login from a new device to verify account");
                let email_token = generate_email_token();
                self.db_client
                    .surreal_client
                    .create_email(
                        &user.id.to_string(),
                        TokenType::Verification,
                        email_token.clone(),
                    )
                    .await?;
                let html = VERIFICATION_EMAIL_HTML
                    .replace("{{username}}", &user.name)
                    .replace("{{email_token}}", &email_token);
                let _email = send_mail(
                    &self.config.mail_server.from_email,
                    vec![&user.email],
                    "Verification",
                    &html,
                    &self.config.mail_server.resend_api_key,
                )
                .await
                .map_err(ExternalError::from)?;
                let response_headers = HeaderMap::new();
                return Ok((
                    response_headers,
                    jar,
                    AppResponse::success(Some("Check your email".to_string()), None),
                ));
            }
        };
        info!("✅ Start creating access_token");
        let access_token = generate_access_token(
            user.id.to_string(),
            self.config.jwt_config.jwt_secret.as_bytes(),
            self.config.jwt_config.jwt_expires_in_seconds,
        )?;
        let mut response_headers = HeaderMap::new();
        response_headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", access_token).parse().unwrap(),
        );
        info!("✅ Start creating refresh token");
        let refresh_token_value = match self
            .db_client
            .surreal_client
            .find_refresh_token_by_user_and_device(&user.id.to_string(), &device.id.to_string())
            .await?
        {
            Some(token) => token.token_value,
            None => {
                let new_token_value = generate_refresh_token();
                self.db_client
                    .surreal_client
                    .create_refresh_token(
                        &user.id.to_string(),
                        &device.id.to_string(),
                        &new_token_value,
                    )
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
        Ok((
            response_headers,
            jar,
            AppResponse::success(
                Some(format!("Login successfully, {}", user.email)),
                Some(LoginResponseData { device }),
            ),
        ))
    }
    pub async fn logout(&self, jar: CookieJar, user: User) -> AppResult<impl IntoResponse + use<>> {
        if let Some(cookie) = jar.get("refresh_token") {
            let refresh_token = cookie.value().to_string();
            match self
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
            AppResponse::success(Some("Logout Success".to_string()), None::<()>),
        ))
    }
    pub async fn verify_user(
        &self,
        headers: HeaderMap,
        addr: SocketAddr,
        payload: VerifyUserRequest,
    ) -> AppResult<impl IntoResponse + use<>> {
        validate_verify_user_payload(&payload)?;
        let user = match self
            .db_client
            .surreal_client
            .find_user_by_email(&payload.email)
            .await?
        {
            Some(user) => user,
            None => return Err(UserErrorKind::UserNotFound.into()),
        };
        let email = match self
            .db_client
            .surreal_client
            .find_verification_email_by_user_id(&user.id.to_string())
            .await?
        {
            Some(email) => email,
            None => return Err(EmailErrorKind::EmailNotFound.into()),
        };
        if email.email_token == payload.token {
            self.db_client
                .surreal_client
                .user_verified(&user.id.to_string())
                .await?;
        }
        let user_agent_str = match headers.get("User-Agent").and_then(|ua| ua.to_str().ok()) {
            Some(user_agent) => user_agent,
            None => return Err(UserErrorKind::MissingUserAgent.into()),
        };
        let (user_agent, os, device) = parse_user_agent_detailed(user_agent_str);
        let new_device = self
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
        Ok(AppResponse::success(
            Some("Verify your account successfully".to_string()),
            new_device,
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
            .surreal_client
            .create_email(
                &user.id.to_string(),
                TokenType::PasswordReset,
                email_token.clone(),
            )
            .await?;
        let html = VERIFICATION_EMAIL_HTML
            .replace("{{username}}", &user.name)
            .replace("{{email_token}}", &email_token);
        let _email = send_mail(
            &self.config.mail_server.from_email,
            vec![&user.email],
            "Reset password",
            &html,
            &self.config.mail_server.resend_api_key,
        )
        .await
        .map_err(ExternalError::from)?;
        Ok(AppResponse::success(
            Some("An reset password email has been sent, please check your email".to_string()),
            None::<()>,
        ))
    }
    pub async fn reset_password(
        &self,
        payload: ResetPasswordRequest,
    ) -> AppResult<impl IntoResponse + use<>> {
        validate_reset_password_payload(&payload)?;
        let user = match self
            .db_client
            .surreal_client
            .find_user_by_email(&payload.email)
            .await?
        {
            Some(user) => user,
            None => return Err(UserErrorKind::UserNotFound.into()),
        };
        let email = match self
            .db_client
            .surreal_client
            .find_reset_password_email_by_user_id(&user.id.to_string())
            .await?
        {
            Some(email) => email,
            None => return Err(EmailErrorKind::EmailNotFound.into()),
        };
        if email.email_token == payload.token {
            self.db_client
                .surreal_client
                .reset_password(&user.id.to_string(), &payload.new_password)
                .await?;
        }
        Ok(AppResponse::success(
            Some("Reset your password successfully".to_string()),
            None::<()>,
        ))
    }
}
