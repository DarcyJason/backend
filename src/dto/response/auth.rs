use serde::Serialize;

use crate::models::device::Device;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub device: Option<Device>,
    pub need_verification: bool,
}

#[derive(Debug, Serialize)]
pub struct VerifyUserResponse {
    pub device: Device,
}
