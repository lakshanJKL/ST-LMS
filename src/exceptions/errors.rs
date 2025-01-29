use thiserror::Error;
use mongodb::error::Error as MongoError;

/// ---------  DB errors -------------------------------

#[derive(Error, Debug)]
pub enum SystemError {
  #[error("MongoDB error: {0}")]
  MongoError(#[from] MongoError),
  #[error("{0} already exists")]
  DuplicateError(String),
  #[error("{0} invalid password, try again")]
  ValidationError(String),
  #[error("{0} not found")]
  NotFoundError(String),
  #[error("{0}")]
  PasswordError(PasswordError),
  #[error("{0}")]
  JwtError(JwtError)
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
  #[error("{0} invalid password, try again")]
  InvalidPassword(String),
}
