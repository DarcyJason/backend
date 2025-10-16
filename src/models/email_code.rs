use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailCode {
    pub id: Thing,
    pub user_id: String,
    pub email: String,
    pub code: String,
    pub code_type: CodeType,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeType {
    Verification,
    PasswordReset,
}
