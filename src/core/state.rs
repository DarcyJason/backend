use std::sync::Arc;

use crate::{
    core::config::AppConfig,
    database::client::DBClient,
    services::Services,
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
    pub services: Services,
}

impl AppState {
    pub fn new(config: AppConfig, db_client: DBClient) -> Self {
        let config = Arc::new(config);
        let db_client = Arc::new(db_client);
        let services = Services::new(config.clone(), db_client.clone());
        AppState {
            config,
            db_client,
            services,
        }
    }
}
