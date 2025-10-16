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
    pub backend_server: BackendServerConfig,
    pub frontend_server: FrontendServerConfig,
    pub mail_server: MailServerConfig,
    pub surreal_server: SurrealServerConfig,
    pub redis_server: RedisServerConfig,
    pub jwt_config: JwtConfig,
}

impl AppConfig {
    pub fn init() -> AppResult<Self> {
        Ok(AppConfig {
            backend_server: BackendServerConfig::init()?,
            frontend_server: FrontendServerConfig::init()?,
            mail_server: MailServerConfig::init()?,
            surreal_server: SurrealServerConfig::init()?,
            redis_server: RedisServerConfig::init()?,
            jwt_config: JwtConfig::init()?,
        })
    }
}
