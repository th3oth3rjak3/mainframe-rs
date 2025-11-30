use std::fmt::Display;

use argon2::{
    PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct Password(String);

impl Password {
    pub fn new(raw_password: &str) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = argon2::Argon2::default();
        let hash = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .unwrap();

        Self(hash.to_string())
    }

    pub fn verify(&self, candidate: &[u8]) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(&self.0) else {
            return false;
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
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: Password,
    pub is_admin: bool,
    pub last_login: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub raw_password: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub raw_password: Option<String>,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_login: Option<OffsetDateTime>,
    pub is_admin: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            username: user.username,
            last_login: user.last_login,
            is_admin: user.is_admin,
        }
    }
}

impl From<CreateUserRequest> for User {
    fn from(request: CreateUserRequest) -> Self {
        Self {
            id: 0,
            first_name: request.first_name,
            last_name: request.last_name,
            email: request.email,
            username: request.username,
            password: Password::new(&request.raw_password),
            last_login: None,
            is_admin: request.is_admin,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use argon2::{
        Argon2,
        password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
    };

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

    #[test]
    fn generate_admin_password_hash() {
        let password = b"admin";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

        println!("Admin password hash: {}", password_hash);
        // Copy this hash to your migration
    }
}
