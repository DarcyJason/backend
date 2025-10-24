use crate::{config::AppConfig, database::client::DBClient};

#[derive(Debug)]
pub struct AppState {
    pub config: AppConfig,
    pub db_client: DBClient,
}

impl AppState {
    pub fn new(config: AppConfig, db_client: DBClient) -> Self {
        AppState { config, db_client }
    }
}
