use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    pub id: Thing,
    pub user_id: String,
    pub token: String,
    pub revoked: bool,
}
