use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: Option<String>,
    pub requires_verification: bool,
}
