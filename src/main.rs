mod models;
mod db;
mod services;
mod controllers;
mod repo;
mod routes;

use actix_web::{web, App, HttpServer};
use actix_web::web::{Data};
use dotenv::dotenv;
use crate::db::mongo::get_mongo_client;
use crate::routes::routes::user_route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // connect to database
    let db = get_mongo_client().await.expect("Failed to connect to database");
    let data = Data::new(db);

    // start the HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(user_route())
    })
        .bind(("localhost", 8001))?;

    println!("Server running on http://localhost:8001");

    server.run().await
}
