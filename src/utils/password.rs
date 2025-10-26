use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;

use crate::custom::errors::validation::ValidationErrorKind;
use crate::custom::result::AppResult;

pub fn hash_password(password: String) -> AppResult<(String, String)> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(ValidationErrorKind::PasswordHashingError)?
        .to_string();
    Ok((hashed_password, salt.to_string()))
}

pub fn compare_hashed_password(password: &str, hashed_password: &str) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(hashed_password)
        .map_err(|err| ValidationErrorKind::PasswordHashingError(err))?;
    let is_password_match = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok_and(|_| true);
    Ok(is_password_match)
}

pub fn validate_password(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_digit())
        && password.chars().any(|c| c.is_ascii_alphabetic())
        && password
            .chars()
            .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
}
