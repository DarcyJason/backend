use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBlacklist {
    pub id: Thing,
    pub user_id: String,
    pub ip: String,
}
