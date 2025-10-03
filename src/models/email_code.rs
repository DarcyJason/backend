use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailCode {
    pub id: Thing,
    pub email: String,
    pub code: String,
    pub code_type: String,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
}
