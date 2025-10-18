use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginResponseData {
    pub access_token: Option<String>,
    pub requires_verification: bool,
}
