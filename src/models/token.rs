use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub token_type: String,
    pub user_id: Thing,
    pub iat: usize,
    pub exp: usize,
}
