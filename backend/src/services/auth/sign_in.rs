use crate::config::db::DB;
use crate::models::dtos::dto_signin::{LoginRequest, LoginResponse};
use crate::models::user::User;
use crate::utils::errors::Error;
use crate::utils::paseto::generate_token;

pub async fn sign_in_service(login_payload: LoginRequest) -> Result<Option<LoginResponse>, Error> {
    // Find user by email
    let mut result = DB
        .query("SELECT * FROM user WHERE email_address = $email")
        .bind(("email", login_payload.email_address.to_string()))
        .await?;
    let users: Vec<User> = result.take(0)?;

    let user = users.into_iter().next().ok_or(Error::EmailNotFound)?;

    // Verify password (replace with proper password hashing verification)
    if !verify_password(&login_payload.password, &user.password) {
        return Err(Error::InvalidCredentials);
    }

    let token = generate_token(&user.id.unwrap().to_string(), &user.email_address);
    Ok(Some(LoginResponse { token }))
}

// Add password verification function
fn verify_password(plain: &str, hashed: &str) -> bool {
    // Implement proper password verification here (e.g., bcrypt)
    plain == hashed // Replace with actual hash verification
}
