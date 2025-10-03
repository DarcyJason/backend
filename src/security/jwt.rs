use crate::{custom::result::AppResult, models::token::TokenClaims};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};

pub fn generate_token(
    token_type: String,
    user_id: String,
    secret: &[u8],
    expires_in_seconds: i64,
) -> AppResult<String> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(expires_in_seconds)).timestamp() as usize;
    let claims = TokenClaims {
        token_type,
        user_id,
        iat,
        exp,
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )?)
}

pub fn validate_token<T: Into<String>>(token: T, secret: &[u8]) -> AppResult<String> {
    let token_data = decode::<TokenClaims>(
        &token.into(),
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims.user_id)
}

pub fn generate_tokens(
    user_id: String,
    secret: &[u8],
    access_expires_in_seconds: i64,
    refresh_expires_in_seconds: i64,
) -> AppResult<(String, String)> {
    let access_token = generate_token(
        "access".to_string(),
        user_id.clone(),
        secret,
        access_expires_in_seconds,
    )?;
    let refresh_token = generate_token(
        "refresh".to_string(),
        user_id,
        secret,
        refresh_expires_in_seconds,
    )?;

    Ok((access_token, refresh_token))
}

pub fn validate_access_token(access_token: String, secret: &[u8]) -> AppResult<String> {
    validate_token(access_token, secret)
}

pub fn validate_refresh_token(refresh_token: String, secret: &[u8]) -> AppResult<String> {
    validate_token(refresh_token, secret)
}

pub fn refresh_access_token(
    refresh_token: String,
    secret: &[u8],
    access_expires_in_seconds: i64,
) -> AppResult<String> {
    let user_id = validate_refresh_token(refresh_token, secret)?;
    generate_token(
        "access".to_string(),
        user_id,
        secret,
        access_expires_in_seconds,
    )
}

pub fn refresh_refresh_token(
    refresh_token: String,
    secret: &[u8],
    refresh_expires_in_seconds: i64,
) -> AppResult<String> {
    let user_id = validate_refresh_token(refresh_token, secret)?;
    generate_token(
        "refresh".to_string(),
        user_id,
        secret,
        refresh_expires_in_seconds,
    )
}
