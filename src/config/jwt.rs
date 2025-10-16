use figment::{Figment, providers::Env};
use serde::{Deserialize, Serialize};

use crate::custom::result::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub jwt_secret: String,
    pub access_token_expires_in_seconds: i64,
    pub refresh_token_expires_in_seconds: i64,
}

impl JwtConfig {
    fn figment() -> Figment {
        Figment::new().merge(Env::prefixed(""))
    }
    pub fn init() -> AppResult<Self> {
        Ok(JwtConfig::figment().extract()?)
    }
}
