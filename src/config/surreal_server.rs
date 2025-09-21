use figment::Figment;
use figment::providers::Env;
use serde::{Deserialize, Serialize};

use crate::custom::result::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealServerConfig {
    pub surreal_host: String,
    pub surreal_root_name: String,
    pub surreal_root_password: String,
    pub surreal_namespace: String,
    pub surreal_database: String,
}

impl SurrealServerConfig {
    fn figment() -> Figment {
        Figment::new().merge(Env::prefixed(""))
    }
    pub fn init() -> AppResult<Self> {
        Ok(SurrealServerConfig::figment().extract()?)
    }
}
