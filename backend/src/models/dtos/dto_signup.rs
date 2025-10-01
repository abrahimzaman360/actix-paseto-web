use crate::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub full_name: String,
    pub username: String,
    pub email_address: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub status_code: u16,
    pub message: String,
}

impl From<RegisterRequest> for User {
    fn from(dto: RegisterRequest) -> Self {
        let user = User {
            id: None,
            full_name: dto.full_name,
            username: dto.username,
            email_address: dto.email_address,
            img: None,
            password: dto.password,
            email_verified: false,        // Default false
            platform_verification: false, // Default false
            created_at: None,             // Autopopulated
            updated_at: None,             // Autopopulated
        };
        user
    }
}
