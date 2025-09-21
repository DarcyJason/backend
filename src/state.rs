use crate::{config::Config, database::client::DBClient};

pub struct AppState {
    pub config: Config,
    pub db_client: DBClient,
}

impl AppState {
    pub fn new(config: Config, db_client: DBClient) -> Self {
        AppState { config, db_client }
    }
}
