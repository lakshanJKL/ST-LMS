mod models;
mod services;
mod controllers;
mod repo;
mod config;
mod utill;
mod midleware;
mod exceptions;


use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::web::{Data};
use dotenv::dotenv;
use log::{debug, error, info, warn};
use crate::config::database::get_mongo_client;
use crate::config::app::{app_config};
use crate::config::governor::rate_limiter;
use crate::midleware::auth::JwtMiddleware;
use crate::midleware::cors::cors;
use crate::midleware::loggers::logger;
use crate::midleware::security_headers::security_headers;
use crate::repo::audit_log_repo::AuditLogRepo;
use crate::repo::user_repo::UserRepo;
use crate::services::user_service::UserService;


#[get("/")]
async fn test() -> impl Responder {
    info!("info: Received a request to the root endpoint");
    warn!("This is a warning message");
    debug!("This is an error message");
    error!("This is an error message");
    HttpResponse::Ok().json("This is test")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // // Initialize repositories & services
    let db = get_mongo_client().await.expect("Failed to connect to database");
    let log_repo = AuditLogRepo::new(&db.clone()).await;
    let user_repo = UserRepo::new(&db.clone(),log_repo).await;
    let user_service_data = UserService::new(user_repo);
    let user_service = Data::new(user_service_data);


    // start the HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(user_service.clone())
            .wrap(cors())
            .wrap(JwtMiddleware)
            .wrap(security_headers())
            .wrap(rate_limiter())
            .wrap(logger())
            .service(test)
            .configure(app_config)
    })
        .bind(("localhost", 8001))?;
    println!("Server running on http://localhost:8001");
    server.run().await
}
