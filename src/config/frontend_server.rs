use figment::Figment;
use figment::providers::Env;
use serde::{Deserialize, Serialize};

use crate::custom::result::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendServerConfig {
    pub frontend_address: String,
}

impl FrontendServerConfig {
    fn figment() -> Figment {
        Figment::new().merge(Env::prefixed(""))
    }
    pub fn init() -> AppResult<Self> {
        Ok(FrontendServerConfig::figment().extract()?)
    }
}
