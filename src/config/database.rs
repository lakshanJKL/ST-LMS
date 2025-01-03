use std::env;
use mongodb::{Client, Database,error::Result};

/// Connect to MongoDB
pub async fn get_mongo_client() ->Result<Database>{
    dotenv::dotenv().ok();

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let db = env::var("DB_NAME").expect("DB_NAME must be set in .env");

    let client = Client::with_uri_str(&mongo_uri).await?;
    Ok(client.database(&db))
}