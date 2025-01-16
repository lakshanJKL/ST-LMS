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
use crate::config::database::get_mongo_client;
use crate::config::app::{app_config};
use crate::midleware::auth::JwtMiddleware;
use crate::repo::user_repo::UserRepo;
use crate::services::user_service::UserService;


#[get("/")]
async fn test() -> impl Responder {
    HttpResponse::Ok().json("This is test")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // // Initialize repositories & services
    let db = get_mongo_client().await.expect("Failed to connect to database");
    let user_repo = UserRepo::new(db.clone()).await;
    let user_service_data = UserService::new(user_repo);
    let user_service = Data::new(user_service_data);


    // start the HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(user_service.clone())
            .wrap(JwtMiddleware)
            .service(test)
            .configure(app_config)
    })
        .bind(("localhost", 8001))?;
    println!("Server running on http://localhost:8001");
    server.run().await
}
