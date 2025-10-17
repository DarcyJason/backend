use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct AppResponse<T>
where
    T: Serialize,
{
    pub status: String,
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> AppResponse<T>
where
    T: Serialize,
{
    pub fn default(message: String, data: Option<T>) -> Self {
        AppResponse {
            status: "success".to_string(),
            code: StatusCode::OK.as_u16(),
            message,
            data,
        }
    }
    pub fn build(status: String, code: u16, message: String, data: Option<T>) -> Self {
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
        let status_code = match StatusCode::from_u16(self.code) {
            Ok(code) => code,
            Err(_) => {
                eprintln!("Invalid status code: {}", self.code);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        let body = AppResponse {
            status: self.status,
            code: self.code,
            message: self.message,
            data: self.data,
        };
        (status_code, Json(body)).into_response()
    }
}
