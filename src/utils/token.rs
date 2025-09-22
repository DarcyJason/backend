use crate::{custom::result::AppResult, models::token::TokenClaims};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use surrealdb::sql::Thing;

pub fn generate_token(
    token_type: String,
    user_id: Thing,
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

pub fn validate_token<T: Into<String>>(token: T, secret: &[u8]) -> AppResult<Thing> {
    let token_data = decode::<TokenClaims>(
        &token.into(),
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims.user_id)
}
