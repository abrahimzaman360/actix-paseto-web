use crate::handlers::protected::home::protected_handler;
use crate::middlewares::auth_middleware::auth_middleware;
use actix_web::web;

pub fn protected_routes() -> actix_web::Scope<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    web::scope("/protected")
        .wrap(actix_web::middleware::from_fn(auth_middleware))
        .route("/user", web::get().to(protected_handler))
        .route("/profile", web::get().to(protected_handler))
        .service(
            web::scope("/admin")
                .route("/dashboard", web::get().to(protected_handler))
                .route("/settings", web::get().to(protected_handler)),
        )
}
