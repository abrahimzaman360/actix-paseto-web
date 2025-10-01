use crate::middlewares::auth_middleware::TokenClaims;
use actix_web::error::ErrorInternalServerError;
use actix_web::{Error, HttpMessage, HttpResponse};

pub async fn protected_handler(req: actix_web::HttpRequest) -> Result<HttpResponse, Error> {
    // Extract claims from request extensions
    let claims = req
        .extensions()
        .get::<TokenClaims>()
        .ok_or_else(|| ErrorInternalServerError("Claims not found"))?
        .clone();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Access granted",
        "user_id": claims.user_id,
        "email": claims.email,
        "roles": claims.roles,
    })))
}