use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrustedDeviceErrorKind {
    #[error("Create trust device failed")]
    CreateTrustedDeviceFailed,
    #[error("Trusted device not found")]
    TrustedDeviceNotFound,
}
