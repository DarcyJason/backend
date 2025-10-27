use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use surrealdb::sql::Thing;
use uuid::Uuid;

use crate::{
    custom::{errors::external::ExternalError, result::AppResult},
    models::token::TokenClaims,
};

pub fn generate_access_token(
    user_id: Thing,
    secret: &[u8],
    expires_in_seconds: i64,
) -> AppResult<String> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::seconds(expires_in_seconds)).timestamp() as usize;
    let claims = TokenClaims { user_id, iat, exp };
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .map_err(ExternalError::from)?)
}

pub fn validate_access_token(token: String, secret: &[u8]) -> AppResult<Thing> {
    let token_data = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(ExternalError::from)?;
    Ok(token_data.claims.user_id)
}

pub fn generate_refresh_token() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_email_token() -> String {
    Uuid::new_v4().simple().to_string()
}
