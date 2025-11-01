use crate::models::user::{User, UserRole, UserStatus};

use chrono::{DateTime, Utc};
use serde::Serialize;
use surrealdb::sql::Thing;

#[derive(Debug, Serialize)]
pub struct MeResponseData {
    pub id: Thing,
    pub name: String,
    pub role: UserRole,
    pub is_verified: bool,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
}

impl From<User> for MeResponseData {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            role: user.role,
            is_verified: user.is_verified,
            status: user.status,
            created_at: user.created_at,
        }
    }
}
