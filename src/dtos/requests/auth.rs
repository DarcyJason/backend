use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyUserRequest {
    pub email: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgetPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub token: String,
    pub new_password: String,
    pub confirm_password: String,
}
