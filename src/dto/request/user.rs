use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserProfileRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
    pub new_confirm_password: String,
}
