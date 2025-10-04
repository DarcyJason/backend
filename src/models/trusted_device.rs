use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct TrustDevice {
    pub id: Thing,
    pub user_id: String,
    pub device: String,
    pub ip: String,
    pub last_login_at: DateTime<Utc>,
}
