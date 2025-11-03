use serde::Serialize;

use crate::models::device::Device;

#[derive(Debug, Serialize)]
pub struct LoginResponseData {
    pub device: Option<Device>,
    pub need_verification: bool,
}

#[derive(Debug, Serialize)]
pub struct VerifyUserResponseData {
    pub device: Device,
}
