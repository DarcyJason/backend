use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub token_type: String,
    pub user_id: String,
    pub iat: usize,
    pub exp: usize,
}
