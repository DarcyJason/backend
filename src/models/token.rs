use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_id: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    pub id: Thing,
    pub user_id: String,
    pub token: String,
    pub revoked: bool,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailVerificationToken {
    pub id: Thing,
    pub user_id: String,
    pub email: String,
    pub token: String,
    pub token_type: TokenType,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Verification,
    PasswordReset,
}
