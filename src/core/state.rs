use std::sync::Arc;

use crate::{
    core::config::AppConfig,
    database::client::DBClient,
    services::{auth::AuthService, health::HealthService, user::UserService},
};

#[derive(Debug)]
pub struct AppStateTemp {
    pub config: AppConfig,
    pub db_client: DBClient,
}

#[derive(Debug)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_client: Arc<DBClient>,
    pub health_service: HealthService,
    pub auth_service: AuthService,
    pub user_service: UserService,
}

impl AppState {
    pub fn new(config: AppConfig, db_client: DBClient) -> Self {
        let config = Arc::new(config);
        let db_client = Arc::new(db_client);
        let health_service = HealthService::new(config.clone(), db_client.clone());
        let auth_service = AuthService::new(config.clone(), db_client.clone());
        let user_service = UserService::new(config.clone(), db_client.clone());
        AppState {
            config,
            db_client,
            health_service,
            auth_service,
            user_service,
        }
    }
}
