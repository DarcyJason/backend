use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;

#[derive(Serialize)]
pub struct AppResponse<T>
where
    T: Serialize,
{
    pub status: String,
    pub code: u16,
    pub message: String,
    pub data: T,
}

impl<T> AppResponse<T>
where
    T: Serialize,
{
    pub fn success(message: String, data: T) -> Self {
        AppResponse {
            status: "success".to_string(),
            code: StatusCode::OK.as_u16(),
            message,
            data,
        }
    }
    pub fn error(code: u16, message: String, data: T) -> Self {
        AppResponse {
            status: "error".to_string(),
            code,
            message,
            data,
        }
    }
    pub fn build(status: String, code: u16, message: String, data: T) -> Self {
        AppResponse {
            status,
            code,
            message,
            data,
        }
    }
}

impl<T> IntoResponse for AppResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match StatusCode::from_u16(self.code) {
            Ok(code) => {
                let status_code = code;
                let body = AppResponse {
                    status: self.status,
                    code: self.code,
                    message: self.message,
                    data: self.data,
                };
                (status_code, Json(body)).into_response()
            }
            Err(_) => {
                error!("❌ Invalid status code: {}", self.code);
                let body = AppResponse {
                    status: "error".to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "Internal Server Error".to_string(),
                    data: None::<()>,
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}
