use actix_web::{web, HttpResponse, Result};
use crate::models::dtos::dto_signin::LoginRequest;
use crate::models::dtos::dto_signup::RegisterRequest;
use crate::services::auth::sign_in::sign_in_service;
use crate::services::auth::sign_up::sign_up_service;
use crate::utils::errors::Error;

pub async fn sign_up_handler(
    user_data: web::Json<RegisterRequest>,
) -> Result<HttpResponse, Error> {
    let response = sign_up_service(user_data.into_inner()).await?;

    Ok(HttpResponse::Ok().json(response))
}

pub async fn sign_in_handler(
    credentials: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    let response = sign_in_service(credentials.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}