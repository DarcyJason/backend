use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub temp_token: String,
    pub requires_verification: bool,
    pub access_token: Option<String>,
}
