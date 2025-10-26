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
use crate::custom::result::AppResult;

pub mod backend_server;
pub mod frontend_server;
pub mod jwt;
pub mod mail_server;
pub mod redis_server;
pub mod surreal_server;

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
        Ok(Figment::new().merge(Env::prefixed("")).extract()?)
    }
}
