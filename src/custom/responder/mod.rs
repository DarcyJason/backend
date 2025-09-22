use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use cookie::Cookie;
use serde::Serialize;
use utoipa::ToSchema;

use crate::custom::errors::AppError;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub status: String,
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
    #[serde(skip_serializing)]
    pub cookies: Vec<Cookie<'static>>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: String, data: T) -> Self {
        ApiResponse {
            status: "success".to_string(),
            code: 200,
            message,
            data: Some(data),
            cookies: Vec::new(),
        }
    }
    pub fn error(code: StatusCode, message: String) -> Self {
        ApiResponse {
            status: "error".to_string(),
            code: code.as_u16(),
            message,
            data: None,
            cookies: Vec::new(),
        }
    }
    pub fn with_cookie(mut self, cookie: Cookie<'static>) -> Self {
        self.cookies.push(cookie);
        self
    }

    pub fn with_cookies<I: IntoIterator<Item = Cookie<'static>>>(mut self, cookies: I) -> Self {
        self.cookies.extend(cookies);
        self
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let mut response = (status, Json(&self)).into_response();
        for cookie in self.cookies {
            response.headers_mut().append(
                axum::http::header::SET_COOKIE,
                cookie.to_string().parse().unwrap(),
            );
        }
        response
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, message) = self.status_code();
        let api_response: ApiResponse<()> = ApiResponse::error(status_code, message);
        (status_code, Json(api_response)).into_response()
    }
}
