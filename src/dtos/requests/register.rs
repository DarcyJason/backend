use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    #[schema(example = "your_strong_password")]
    pub password: String,
    #[schema(example = "your_strong_password")]
    pub confirm_password: String,
}
