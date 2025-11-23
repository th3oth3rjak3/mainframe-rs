use std::fmt::Display;

use argon2::{
    PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(transparent)]
pub struct Password(String);

impl Password {
    pub fn new(raw_password: &str) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = argon2::Argon2::default();
        let hash = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .unwrap();

        Password(hash.to_string())
    }

    pub fn verify(&self, candidate: &[u8]) -> bool {
        let parsed_hash = match PasswordHash::new(&self.0) {
            Ok(h) => h,
            Err(_) => return false,
        };

        let argon2 = argon2::Argon2::default();
        argon2.verify_password(candidate, &parsed_hash).is_ok()
    }
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: Password,
    pub last_login: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub raw_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub raw_password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub last_login: Option<OffsetDateTime>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            username: user.username,
            last_login: user.last_login,
        }
    }
}

impl From<CreateUserRequest> for User {
    fn from(request: CreateUserRequest) -> Self {
        User {
            id: 0,
            first_name: request.first_name,
            last_name: request.last_name,
            email: request.email,
            username: request.username,
            password: Password::new(&request.raw_password),
            last_login: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::users::Password;

    #[test]
    pub fn test_creating_password_not_raw_string() {
        let raw_pw = "hunter2";
        let pw = Password::new(raw_pw);
        assert_ne!(pw.0, String::from(raw_pw));
    }

    #[test]
    pub fn password_verification_succeeds_when_same() {
        let raw_pw = "hunter2";
        let pw = Password::new(raw_pw);
        let valid = pw.verify(b"hunter2");
        assert!(valid);
    }

    #[test]
    pub fn password_verification_fails_when_different() {
        let raw_pw = "hunter2";
        let pw = Password::new(raw_pw);
        let valid = pw.verify(b"hunter123");
        assert!(!valid);
    }
}
