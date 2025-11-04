use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EmailVerificationDTO {
    pub acceess_token: String,
    pub code_type: String,
    pub verification_code: String,
}
