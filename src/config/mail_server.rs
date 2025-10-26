use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailServerConfig {
    pub from_email: String,
    pub resend_api_key: String,
}
