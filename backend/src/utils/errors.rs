use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    Db(String),
    #[error("User already exists")]
    UserExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Email not found")]
    EmailNotFound,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Db(e) => HttpResponse::InternalServerError().body(e.to_string()),
            Error::UserExists => HttpResponse::Conflict().body("User already exists"),
            Error::InvalidCredentials => HttpResponse::Unauthorized().body("Invalid credentials"),
            Error::EmailNotFound => HttpResponse::NotFound().body("Email not found"),
        }
    }
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db(error.to_string())
    }
}
