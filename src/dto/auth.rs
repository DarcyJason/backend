use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterDTO {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyUserDTO {
    pub email: String,
    pub email_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgetPasswordDTO {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordDTO {
    pub email: String,
    pub token: String,
    pub new_password: String,
    pub confirm_password: String,
}
