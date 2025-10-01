mod config;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod services;
mod utils;

use actix_cors::Cors;
use crate::config::db::DB;
use crate::routes::auth::auth_routes::auth_routes;
use crate::routes::protected::home_routes::protected_routes;
use actix_web::{App, HttpServer, web};
use base64::Engine;
use base64::engine::general_purpose;
use dotenvy::dotenv;
use env_logger::Env;
use pasetors::keys::SymmetricKey;
use pasetors::version4::V4;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load Envs
    dotenv().expect("Failed to load .env file");

    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // DB Turn
    DB.connect::<Ws>("127.0.0.1:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "secret",
    })
    .await?;

    DB.use_ns("tea_social").use_db("authentication").await?;

    // DB Query:
    DB.query(
        "
        -- Define SchemaFull user table
        DEFINE TABLE user SCHEMAFULL;

        -- Define fields matching your struct
        DEFINE FIELD full_name ON TABLE user TYPE string;
        DEFINE FIELD username ON TABLE user TYPE string;
        DEFINE FIELD email_address ON TABLE user TYPE string
          ASSERT string::is::email($value);
        DEFINE FIELD img ON TABLE user TYPE option<string>;
        DEFINE FIELD password ON TABLE user TYPE string;

        -- Other fields
        DEFINE FIELD email_verified ON TABLE user TYPE bool;
        DEFINE FIELD platform_verification ON TABLE user TYPE bool;

        -- TimeStamps:
        DEFINE FIELD created_at ON TABLE user VALUE time::now() READONLY;
        DEFINE FIELD updated_at ON TABLE user VALUE time::now();

        -- Define indexes for performance and uniqueness
        DEFINE INDEX unique_user_name ON TABLE user COLUMNS username UNIQUE;
        DEFINE INDEX unique_email ON TABLE user COLUMNS email_address UNIQUE;",
    )
    .await?;

    // Get Key from .Env
    let key_b64 = std::env::var("PASETO_KEY").expect("Missing PASETO_KEY");
    let key_bytes = general_purpose::STANDARD
        .decode(key_b64)
        .expect("Invalid base64 key");
    let key = SymmetricKey::<V4>::from(&key_bytes).expect("Invalid key bytes");

    // Wrap in Data for sharing across workers
    let key_data = web::Data::new(key);

    // Logger:
    log::info!("Starting Actix Web server...");

    // Actix Server in Action!
    HttpServer::new(move || {
        App::new() // Logs each HTTP request
            .wrap(
                Cors::default()
                    .allow_any_origin() // allow all origins (for dev only!)
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
            )
            .app_data(key_data.clone())
            .service(auth_routes())
            .service(protected_routes())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await?;

    Ok(())
}
