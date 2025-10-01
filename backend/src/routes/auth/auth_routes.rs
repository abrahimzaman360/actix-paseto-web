use crate::handlers::auth::handle_authentication::{sign_in_handler, sign_up_handler};
use actix_web::web;

pub fn auth_routes() -> actix_web::Scope {
    web::scope("/auth")
        .route("/sign-up", web::post().to(sign_up_handler))
        .route("/sign-in", web::post().to(sign_in_handler))
}
