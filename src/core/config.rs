use figment::Figment;
use figment::providers::Env;
use serde::{Deserialize, Serialize};

use crate::config::jwt::JwtConfig;
use crate::config::redis_server::RedisServerConfig;
use crate::config::surreal_server::SurrealServerConfig;
use crate::config::{
    backend_server::BackendServerConfig, frontend_server::FrontendServerConfig,
    mail_server::MailServerConfig,
};
use crate::core::error::external::ExternalError;
use crate::core::result::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(flatten)]
    pub backend_server: BackendServerConfig,
    #[serde(flatten)]
    pub frontend_server: FrontendServerConfig,
    #[serde(flatten)]
    pub mail_server: MailServerConfig,
    #[serde(flatten)]
    pub surreal_server: SurrealServerConfig,
    #[serde(flatten)]
    pub redis_server: RedisServerConfig,
    #[serde(flatten)]
    pub jwt_config: JwtConfig,
}

impl AppConfig {
    pub fn init() -> AppResult<Self> {
        Ok(Figment::new()
            .merge(Env::prefixed(""))
            .extract()
            .map_err(ExternalError::from)?)
    }
}
