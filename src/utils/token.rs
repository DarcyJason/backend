use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use uuid::Uuid;

use crate::{custom::result::AppResult, models::token::TokenClaims};

pub fn generate_access_token(
    user_id: String,
    secret: &[u8],
    expires_in_seconds: i64,
) -> AppResult<String> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(expires_in_seconds)).timestamp() as usize;
    let claims = TokenClaims { user_id, iat, exp };
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )?)
}

pub fn validate_access_token(token: String, secret: &[u8]) -> AppResult<String> {
    let token_data = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims.user_id)
}

pub fn generate_refresh_token() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_verification_token() -> String {
    Uuid::new_v4().simple().to_string()
}
