use serde::Deserialize;

use crate::models::trust_device::TrustDevice;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub trust_device: TrustDevice,
}
