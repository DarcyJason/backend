use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealServerConfig {
    pub surreal_host: String,
    pub surreal_root_name: String,
    pub surreal_root_password: String,
    pub surreal_namespace: String,
    pub surreal_database: String,
}
