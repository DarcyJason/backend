use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendServerConfig {
    pub backend_port: u16,
}
