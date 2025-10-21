use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBlacklist {
    pub id: Thing,
    pub ip: String,
    pub reason: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}
