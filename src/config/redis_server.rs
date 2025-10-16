use figment::Figment;
use figment::providers::Env;
use serde::{Deserialize, Serialize};

use crate::custom::result::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisServerConfig {
    pub redis_address: String,
}

impl RedisServerConfig {
    fn figment() -> Figment {
        Figment::new().merge(Env::prefixed(""))
    }
    pub fn init() -> AppResult<Self> {
        Ok(RedisServerConfig::figment().extract()?)
    }
}
