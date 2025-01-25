use thiserror::Error;

/// ---------  validation error -------------------------------
#[derive(Debug,Error)]
pub enum UserServiceError{
  #[error("Validation failed : {0}")]
  ValidationError(String)
}