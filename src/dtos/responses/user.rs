use chrono::{DateTime, Utc};
use serde::Serialize;
use surrealdb::sql::Thing;

use crate::models::user::User;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
