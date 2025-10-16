use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceErrorKind {
    #[error("Create trust device failed")]
    CreateDeviceFailed,
    #[error("Trusted device not found")]
    DeviceNotFound,
}
