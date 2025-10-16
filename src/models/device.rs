use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    pub id: Thing,
    pub user_id: String,
    pub ip: String,
    pub user_agent_family: String,
    pub user_agent_major: String,
    pub user_agent_minor: String,
    pub user_agent_patch: String,
    pub user_agent_patch_minor: String,
    pub os_family: String,
    pub os_major: String,
    pub os_minor: String,
    pub os_patch: String,
    pub device_family: String,
    pub device_brand: String,
    pub device_model: String,
    pub is_trusted: bool,
    pub last_login_at: DateTime<Utc>,
}
