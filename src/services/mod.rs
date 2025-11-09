use std::sync::Arc;

use resend_rs::Resend;

use crate::{
    core::config::AppConfig,
    database::client::DBClient,
    services::{auth::AuthService, health::HealthService, user::UserService},
};

pub mod admin;
pub mod auth;
pub mod health;
pub mod user;

#[derive(Debug)]
pub struct Services {
    pub health: HealthService,
    pub auth: AuthService,
    pub user: UserService,
    pub resend: Arc<Resend>,
}

impl Services {
    pub fn new(config: Arc<AppConfig>, db_client: Arc<DBClient>) -> Self {
        let resend = Arc::new(Resend::new(&config.mail_server.resend_api_key));
        let health = HealthService::new(config.clone(), db_client.clone());
        let auth = AuthService::new(config.clone(), db_client.clone(), resend.clone());
        let user = UserService::new(config, db_client);
        Self {
            health,
            auth,
            user,
            resend,
        }
    }
}
