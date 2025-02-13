use sea_orm::DbErr;
use thiserror::Error;

/// ---------  DB errors -------------------------------

#[derive(Error, Debug)]
pub enum SystemError {
    #[error("Database Error : {0}")]
    DbError(#[from] DbErr),

    #[error("{0} already exists")]
    DuplicateError(String),

    #[error("{0} invalid password, try again")]
    ValidationError(String),

    #[error("{0} not found")]
    NotFoundError(String),

    #[error("Password error: {0}")]
    PasswordError(#[from] PasswordError),

    #[error("JWT Error: {0}")]
    JwtError(#[from] JwtError),
}


/// ---------  JWT errors -------------------------------
#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Invalid token: {0}")]
    TokenError(String),
}

/// ---------  Password errors -------------------------------
#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Invalid password, try again")]
    InvalidPassword,
    #[error("Password hashing failed, Error: {0}")]
    PasswordHashErr(String),
}

