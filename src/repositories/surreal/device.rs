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
pub trait DeviceRepository {
    async fn create_device(
        &self,
        user_id: &str,
        user_agent: String,
        os: String,
        device: String,
        ip: String,
    ) -> AppResult<Device>;
    async fn distrust_device(&self, device_id: &str, user_id: &str) -> AppResult<()>;
    async fn find_trusted_devices_by_user_id(&self, user_id: &str) -> AppResult<Vec<Device>>;
    async fn find_device_by_id(&self, device_id: &str) -> AppResult<Option<Device>>;
}

#[async_trait]
impl DeviceRepository for SurrealClient {
    async fn create_device(
        &self,
        user_id: &str,
        user_agent: String,
        os: String,
        device: String,
        ip: String,
    ) -> AppResult<Device> {
        let sql = r#"
            CREATE devices CONTENT {
                id: rand::uuid::v4(),
                user_id: $user_id,
                user_agent: $user_agent,
                os: $os,
                device: $device,
                ip: $ip,
                is_trusted: true
            }
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id.to_string()))
            .bind(("user_agent", user_agent))
            .bind(("os", os))
            .bind(("device", device))
            .bind(("ip", ip))
            .await?;
        let device: Option<Device> = result.take(0)?;
        match device {
            Some(device) => Ok(device),
            None => Err(AppError::DeviceError(DeviceErrorKind::CreateDeviceFailed)),
        }
    }
    async fn distrust_device(&self, device_id: &str, user_id: &str) -> AppResult<()> {
        let sql = r#"
            UPDATE devices SET is_trusted = false WHERE id = <record> $device_id AND user_id = <record> $user_id
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("device_id", device_id.to_string()))
            .bind(("user_id", user_id.to_string()))
            .await?;
        let updated_device: Option<Device> = result.take(0)?;
        match updated_device {
            Some(_) => Ok(()),
            None => Err(AppError::DeviceError(DeviceErrorKind::DeviceNotFound)),
        }
    }
    async fn find_trusted_devices_by_user_id(&self, user_id: &str) -> AppResult<Vec<Device>> {
        let sql = r#"
            SELECT * FROM devices WHERE user_id = <record> $user_id AND is_trusted = true
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("user_id", user_id.to_string()))
            .await?;
        let trusted_device: Option<Vec<Device>> = result.take(0)?;
        match trusted_device {
            Some(trusted_device) => Ok(trusted_device),
            None => {
                return Err(AppError::DeviceError(DeviceErrorKind::DeviceNotFound));
            }
        }
    }

    async fn find_device_by_id(&self, device_id: &str) -> AppResult<Option<Device>> {
        let sql = r#"
            SELECT * FROM devices WHERE id = <record> $device_id
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("device_id", device_id.to_string()))
            .await?;
        let device: Option<Device> = result.take(0)?;
        Ok(device)
    }
}
