use super::api::{AppResponse, AppResponseBody};
use axum::http::StatusCode;
use serde::Serialize;

pub struct ResponseBuilder<T>
where
    T: Serialize,
{
    code: u16,
    message: String,
    status: String,
    data: Option<T>,
}

impl<T> ResponseBuilder<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            message: "Success".to_string(),
            status: "OK".to_string(),
            data: None,
        }
    }

    pub fn code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = status.into();
        self
    }

    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build_success(self) -> AppResponse<T> {
        AppResponse::Success {
            success: AppResponseBody {
                code: self.code,
                message: self.message,
                status: self.status,
                data: self.data,
            },
        }
    }

    pub fn build_error(self) -> AppResponse<T> {
        AppResponse::Error {
            error: AppResponseBody {
                code: self.code,
                message: self.message,
                status: self.status,
                data: None,
            },
        }
    }
}

impl<T: Serialize> Default for ResponseBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}
