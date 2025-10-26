use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub jwt_secret: String,
    pub jwt_expires_in_seconds: i64,
}
