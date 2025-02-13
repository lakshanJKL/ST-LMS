mod models;
mod services;
mod controllers;
mod config;
mod utill;
mod midleware;
mod exceptions;
mod repo;

use actix_web::{App, HttpServer, Responder};
use actix_web::web::{Data};
use dotenv::dotenv;
use crate::config::database::{establish_connection};
use crate::config::app::{app_config};
use crate::config::governor::rate_limiter;
use crate::midleware::auth::JwtMiddleware;
use crate::midleware::cors::cors;
use crate::midleware::loggers::logger;
use crate::midleware::security_headers::security_headers;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Initialize db  connection
    let db = establish_connection().await;
    let db_data =  Data::new(db);

    // start the HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .wrap(cors())
            .wrap(JwtMiddleware)
            .wrap(security_headers())
            .wrap(rate_limiter())
            .wrap(logger())
            .configure(app_config)

    })
        .bind(("localhost", 8080))?;
    println!("Server running on http://localhost:8080");
    server.run().await
}
