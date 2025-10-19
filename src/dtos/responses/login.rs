use serde::Serialize;

use crate::models::device::Device;

#[derive(Debug, Serialize)]
pub struct LoginResponseData {
    pub device: Device,
}
