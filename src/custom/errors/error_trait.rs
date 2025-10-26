use axum::http::StatusCode;

pub trait ErrorKind: std::error::Error + Send + Sync {
    fn status_code(&self) -> StatusCode;
    fn message(&self) -> String;
}
