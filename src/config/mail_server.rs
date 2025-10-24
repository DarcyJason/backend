use figment::{Figment, providers::Env};
use serde::{Deserialize, Serialize};

use crate::custom::result::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailServerConfig {
    pub from_email: String,
    pub resend_api_key: String,
}

impl MailServerConfig {
    fn figment() -> Figment {
        Figment::new().merge(Env::prefixed(""))
    }
    pub fn init() -> AppResult<Self> {
        Ok(MailServerConfig::figment().extract()?)
    }
}
