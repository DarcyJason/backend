use figment::Figment;
use figment::providers::Env;
use serde::{Deserialize, Serialize};

use crate::custom::result::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendServerConfig {
    pub backend_port: u16,
}

impl BackendServerConfig {
    fn figment() -> Figment {
        Figment::new().merge(Env::prefixed(""))
    }
    pub fn init() -> AppResult<Self> {
        Ok(BackendServerConfig::figment().extract()?)
    }
}
