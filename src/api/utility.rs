use argon2::Argon2;
use argon2::password_hash::{Error, SaltString, rand_core::OsRng};
use argon2::{PasswordHash, PasswordVerifier, PasswordHasher};
use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(password.as_bytes(), &salt)?;
    
    Ok(hashed_password.to_string())
}

