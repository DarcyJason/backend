use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::custom::errors::app_error::AppError;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub status: String,
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: String, data: T) -> Self {
        ApiResponse {
            status: "success".to_string(),
            code: 200,
            message,
            data: Some(data),
        }
    }
    pub fn error(code: StatusCode, message: String) -> Self {
        ApiResponse {
            status: "error".to_string(),
            code: code.as_u16(),
            message,
            data: None,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match StatusCode::from_u16(self.code) {
            Ok(code) => (code, Json(self)).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, message) = self.status_code();
        let api_response: ApiResponse<()> = ApiResponse::error(status_code, message);
        (status_code, Json(api_response)).into_response()
    }
}
