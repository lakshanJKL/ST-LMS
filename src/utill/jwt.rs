use chrono::{Duration, Utc};
use std::env;
use actix_web::dev::ServiceRequest;
use actix_web::{HttpMessage, HttpRequest};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde::de::Error;
use crate::midleware::permission::{has_permission, Permission, Role};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // Subject (username)
    pub roles: Vec<String>,      // role (user role)
    pub exp: usize,        // Expiration timestamp
}


impl Claims {
    pub fn new(use_email: String, user_roles: Vec<String>) -> Self {
        let expiration = Utc::now() + Duration::hours(24); // Token valid for 24 hours
        Self {
            sub: use_email.to_string(),
            roles: user_roles,
            exp: expiration.timestamp() as usize,
        }
    }
}

// Generate a JWT token
pub fn create_token(user_email: String, user_role: Vec<String>) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("SECRET_KEY").expect("JWT_SECRET must be set");
    let claims = Claims::new(user_email, user_role);
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

// verify a JWT token
pub fn verify_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = env::var("SECRET_KEY").expect("JWT_SECRET must be set");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}

// get token
fn extract_token(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|auth_header| auth_header.to_str().ok())
        .filter(|auth_str| auth_str.starts_with("Bearer "))
        .map(|auth_str| auth_str[7..].to_string())
}


// check role from token
pub fn extract_and_check_role_from_token(req: &ServiceRequest) -> bool {
    // Create a HttpRequest from ServiceRequest
    let http = req.request();

    if let Some(token_String) = extract_token(http) {
        let token = token_String.as_ref();

        if let Ok(data) = verify_token(token) {
            let claims = data.claims;

            let allowed_roles = vec![Role::Admin, Role::User, Role::Guest];
            return claims.roles.iter()
                .filter_map(|r| Role::from_str(r)) // Parse string to Role
                .any(|role| allowed_roles.contains(&role));
        }
    }
    false
}


// Check if a user has any of the specified roles

fn has_any_role(req: &HttpRequest, allowed_roles: &Vec<Role>) -> bool {
    if let Some(token_string) = extract_token(req) {
        if let Ok(data) = verify_token(&token_string) {
            let claims = data.claims;

            // Convert String roles in the token to Role enums
            let user_roles: Vec<Role> = claims
                .roles
                .iter()
                .filter_map(|role| Role::from_str(role)) // Parse string to Role
                .collect();

            // Check if the user has at least one allowed role
            return allowed_roles.iter().any(|role| user_roles.contains(role));
        }
    }
    false
}


// Check if a user has a specific permission
pub fn has_permission_with_roles(req: &HttpRequest, roles: &Vec<Role>, permission: &Permission) -> bool {
    if has_any_role(req, roles) {
        // Check permissions if the role is allowed
        if roles.iter().any(|role| has_permission(&role, &permission)) {
            return true;
        }
    }
    false
}



