use std::sync::Arc;

use crate::{config::AppConfig, database::client::DBClient, services::auth::AuthService};

#[derive(Debug)]
pub struct AppStateTemp {
    pub config: AppConfig,
    pub db_client: DBClient,
}

#[derive(Debug)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_client: Arc<DBClient>,
    pub auth_service: AuthService,
}

impl AppState {
    pub fn new(config: AppConfig, db_client: DBClient) -> Self {
        let config = Arc::new(config);
        let db_client = Arc::new(db_client);
        let auth_service = AuthService::new(config.clone(), db_client.clone());
        AppState {
            config,
            db_client,
            auth_service,
        }
    }
}
