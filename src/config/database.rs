use std::env;
use log::{error, info};
use sea_orm::{Database, DatabaseConnection};


/// Connect to MongoDB
pub async fn establish_connection() -> DatabaseConnection {
    dotenv::dotenv().ok();

    let db_uri = env::var("DB_URI").expect("DB_URI must be set");

    println!("DB URI: {}", &db_uri);
    info!("successfully db connection");
    match Database::connect(&db_uri).await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to establish database connection: {}", e);
            panic!("Exiting due to database connection error");
        }
    }
}
