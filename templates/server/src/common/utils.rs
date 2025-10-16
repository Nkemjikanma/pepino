use super::errors::AppError;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
}
pub struct PasswordUtils {}
impl PasswordUtils {
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::PasswordHashingFailed)?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify_password(password: &str, hash: &str) -> bool {
        let password_hash = match PasswordHash::new(&hash) {
            Ok(hash) => hash,
            Err(_) => return false,
        };

        Argon2::default()
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok()
    }
}
