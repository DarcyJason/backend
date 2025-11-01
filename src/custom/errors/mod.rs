use axum::{
    Json,
    response::{IntoResponse, Response},
};

use crate::custom::{errors::error_trait::ErrorKind, response::AppResponse};

pub mod access_token;
pub mod device;
pub mod email;
pub mod error_trait;
pub mod external;
pub mod other;
pub mod refresh_token;
pub mod user;
pub mod validation;

pub struct AppError {
    kind: Box<dyn ErrorKind>,
}

impl<E> From<E> for AppError
where
    E: ErrorKind + 'static,
{
    fn from(err: E) -> Self {
        Self {
            kind: Box::new(err),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}
impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.kind.status_code();
        let message = self.kind.message();
        let body = AppResponse::<()>::error(status_code.as_u16(), &message, status_code.as_str());
        (status_code, Json(body)).into_response()
    }
}
