use serde::Serialize;

use crate::models::device::Device;

#[derive(Debug, Serialize)]
pub struct LoginVO {
    pub device: Option<Device>,
    pub need_verification: bool,
}

#[derive(Debug, Serialize)]
pub struct VerifyUserVO {
    pub device: Device,
}
