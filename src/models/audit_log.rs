use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Thing,
    pub actor: String,
    pub action: String,
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub details: Option<Detail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detail {}
