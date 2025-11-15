use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub id: Thing,
    pub user_id: Thing,
    pub email_type: EmailType,
    pub email_token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_used: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EmailType {
    Verification,
    PasswordReset,
}

impl std::fmt::Display for EmailType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailType::Verification => write!(f, "verification"),
            EmailType::PasswordReset => write!(f, "password_reset"),
        }
    }
}
