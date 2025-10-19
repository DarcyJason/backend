use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    pub id: Thing,
    pub user_id: String,
    pub ip: String,
    pub user_agent: String,
    pub os: String,
    pub device: String,
    pub is_trusted: bool,
    pub last_login_at: DateTime<Utc>,
}
