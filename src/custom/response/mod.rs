use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppResponseBody<T>
where
    T: Serialize,
{
    pub code: u16,
    pub message: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum AppResponse<T>
where
    T: Serialize,
{
    Success { success: AppResponseBody<T> },
    Error { error: AppResponseBody<T> },
}

impl<T> IntoResponse for AppResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match &self {
            AppResponse::Success { success } => (
                StatusCode::from_u16(success.code).unwrap_or(StatusCode::OK),
                Json(self),
            )
                .into_response(),
            AppResponse::Error { error } => (
                StatusCode::from_u16(error.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                Json(self),
            )
                .into_response(),
        }
    }
}

impl<T> AppResponse<T>
where
    T: Serialize,
{
    pub fn success(code: u16, message: &str, status: &str, data: Option<T>) -> Self {
        AppResponse::Success {
            success: AppResponseBody {
                code,
                message: message.to_string(),
                status: status.to_string(),
                data,
            },
        }
    }

    pub fn error(code: u16, message: &str, status: &str) -> Self {
        AppResponse::Error {
            error: AppResponseBody {
                code,
                message: message.to_string(),
                status: status.to_string(),
                data: None,
            },
        }
    }
}
