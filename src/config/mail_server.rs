use figment::{Figment, providers::Env};
use serde::{Deserialize, Serialize};

use crate::custom::result::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailServerConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_address: String,
}

impl MailServerConfig {
    fn figment() -> Figment {
        Figment::new().merge(Env::prefixed(""))
    }
    pub fn init() -> AppResult<Self> {
        Ok(MailServerConfig::figment().extract()?)
    }
}
