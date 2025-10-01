use crate::config::db::DB;
use crate::models::dtos::dto_signup::{RegisterRequest, RegisterResponse};
use crate::models::user::User;
use crate::utils::errors::Error;
use serde::Deserialize;

pub async fn sign_up_service(
    user_data: RegisterRequest,
) -> Result<Option<RegisterResponse>, Error> {
    // Check if user exists first
    let mut result = DB
        .query("SELECT count() FROM user WHERE email_address = $email GROUP ALL")
        .bind(("email", user_data.email_address.to_string()))
        .await?;
    let count_result: Option<CountResult> = result.take(0)?;
    let exists = count_result.map(|c| c.count).unwrap_or(0) > 0;

    if exists {
        return Err(Error::UserExists);
    }

    // Convert to User struct and create
    let user = User::from(user_data);
    let _: Option<User> = DB.create("user").content(user).await?;

    Ok(Some(RegisterResponse {
        status_code: 200,
        message: "User created successfully".to_string(),
    }))
}

#[derive(Deserialize)]
struct CountResult {
    count: i64,
}
