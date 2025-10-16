use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub status: String,
    pub code: u16,
    pub message: String,
}
