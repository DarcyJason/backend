use std::sync::Arc;

use crate::{core::config::AppConfig, database::client::DBClient};

#[derive(Debug)]
pub struct AdminService {
    pub config: Arc<AppConfig>,
    pub db_client: Arc<DBClient>,
}

impl AdminService {
    pub fn new(config: Arc<AppConfig>, db_client: Arc<DBClient>) -> Self {
        Self { config, db_client }
    }
}
