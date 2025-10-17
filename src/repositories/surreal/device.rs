use async_trait::async_trait;

use crate::{
    custom::{
        errors::{AppError, device::DeviceErrorKind},
        result::AppResult,
    },
    database::surreal::client::SurrealClient,
    models::device::Device,
};

#[async_trait]
pub trait TrustedDeviceRepository {
    async fn create_trusted_device(
        &self,
        user_id: &str,
        device: String,
        ip: String,
    ) -> AppResult<()>;
    async fn find_trusted_device_by_email(&self, email: &str) -> AppResult<Device>;
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
            CREATE devices CONTENT {
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
        let device: Option<Device> = result.take(0)?;
        match device {
            Some(_) => Ok(()),
            None => Err(AppError::DeviceError(DeviceErrorKind::CreateDeviceFailed)),
        }
    }
    async fn find_trusted_device_by_email(&self, email: &str) -> AppResult<Device> {
        let sql = r#"
            SELECT * FROM devices WHERE email = $email LIMIT 1
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("email", email.to_string()))
            .await?;
        let trusted_device: Option<Device> = result.take(0)?;
        match trusted_device {
            Some(trusted_device) => Ok(trusted_device),
            None => {
                return Err(AppError::DeviceError(DeviceErrorKind::DeviceNotFound));
            }
        }
    }
}
