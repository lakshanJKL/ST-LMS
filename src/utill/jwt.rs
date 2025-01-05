use chrono::{Duration, Utc};
use std::env;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde::de::Error;

#[derive(Debug,Serialize,Deserialize)]
pub struct Claims{
   pub sub:String,      // Subject (user ID)
   pub exp:usize        // Expiration timestamp
}

impl Claims {
    pub fn new(use_id:String)->Self{
        let expiration = Utc::now()+ Duration::hours(24); // Token valid for 24 hours
        Self{
            sub:use_id,
            exp: expiration.timestamp() as usize,
        }
    }
}

// Generate a JWT token
pub fn create_token(user_id:String)->Result<String,jsonwebtoken::errors::Error>{
    let secret = env::var("SECRET_KEY").expect("JWT_SECRET must be set");
    let claims = Claims::new(user_id);
    encode(&Header::default(),&claims,&EncodingKey::from_secret(secret.as_ref()))
}

// verify a JWT token
pub fn verify_token(token:&str)-> Result<TokenData<Claims>,jsonwebtoken::errors::Error>{
    let secret = env::var("SECRET_KEY").expect("JWT_SECRET must be set");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    )
}


