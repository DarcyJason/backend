use async_trait::async_trait;

use crate::{
    custom::{
        errors::{AppError, trusted_device::TrustedDeviceErrorKind},
        result::AppResult,
    },
    database::surreal::client::SurrealClient,
    models::trusted_device::TrustDevice,
};

#[async_trait]
pub trait TrustedDeviceRepository {
    async fn create_trusted_device(
        &self,
        user_id: &str,
        device: String,
        ip: String,
    ) -> AppResult<()>;
}

#[async_trait]
impl TrustedDeviceRepository for SurrealClient {
    async fn create_trusted_device(
        &self,
        user_id: &str,
        device: String,
        ip: String,
    ) -> AppResult<()> {
        let sql = r#"
            CREATE trusted_devices CONTENT {
                id: rand::uuid::v4(),
                user_id: $user_id,
                device: $device,
                ip: $ip,
            }
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id.to_string()))
            .bind(("device", device))
            .bind(("ip", ip))
            .await?;
        let device: Option<TrustDevice> = result.take(0)?;
        match device {
            Some(_) => Ok(()),
            None => Err(AppError::TrustedDeviceError(
                TrustedDeviceErrorKind::CreateTrustDeviceFailed,
            )),
        }
    }
}
