use argon2::{Argon2, PasswordHasher, password_hash::SaltString, PasswordHash, PasswordVerifier};
use rand_core::OsRng; // Secure random number generator

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Use the default Argon2 configuration
    let argon2 = Argon2::default();

    // Hash the password
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

    // Return the hashed password as a string
    Ok(password_hash.to_string())
}


pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, argon2::password_hash::Error> {
    // Parse the hashed password into a `PasswordHash` struct
    let parsed_hash = PasswordHash::new(hashed_password)?;

    // Use the Argon2 default configuration for verification
    let argon2 = Argon2::default();

    // Verify the password against the parsed hash
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
