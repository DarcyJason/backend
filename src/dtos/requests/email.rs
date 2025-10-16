use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EmailVerificationRequest {
    pub temp_token: String,
    pub verification_code: String,
}
