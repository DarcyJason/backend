use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendServerConfig {
    pub frontend_address: String,
}
