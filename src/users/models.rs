use anyhow::anyhow;
use argon2::{
    PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::fmt::Display;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::roles::{Role, RoleName};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct Password(String);

impl Password {
    pub fn new(raw_password: &str) -> Result<Self, anyhow::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = argon2::Argon2::default();
        let hash = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .map_err(|err| anyhow!(err))?;

        Ok(Self(hash.to_string()))
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
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: Password,
    pub last_login: Option<OffsetDateTime>,
    pub failed_login_attempts: i64,
    pub last_failed_login_attempt: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub is_disabled: bool,
    pub roles: Vec<Role>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserBase {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: Password,
    pub last_login: Option<OffsetDateTime>,
    pub failed_login_attempts: i64,
    pub last_failed_login_attempt: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub is_disabled: bool,
}

impl From<UserBase> for User {
    fn from(value: UserBase) -> Self {
        Self {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            username: value.username,
            password_hash: value.password_hash,
            last_login: value.last_login,
            failed_login_attempts: value.failed_login_attempts,
            last_failed_login_attempt: value.last_failed_login_attempt,
            created_at: value.created_at,
            updated_at: value.updated_at,
            roles: Vec::new(),
            is_disabled: value.is_disabled,
        }
    }
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
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_login: Option<OffsetDateTime>,
    pub roles: Vec<Role>,
}

impl UserResponse {
    pub fn is_admin(&self) -> bool {
        for r in &self.roles {
            if r.name == RoleName::Administrator {
                return true;
            }
        }

        false
    }
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
            roles: user.roles,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBaseResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_login: Option<OffsetDateTime>,
}

impl From<UserBase> for UserBaseResponse {
    fn from(user: UserBase) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            username: user.username,
            last_login: user.last_login,
        }
    }
}

impl TryFrom<CreateUserRequest> for User {
    type Error = anyhow::Error;

    fn try_from(request: CreateUserRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::now_v7(),
            first_name: request.first_name,
            last_name: request.last_name,
            email: request.email,
            username: request.username,
            password_hash: Password::new(&request.raw_password)?,
            last_login: None,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
            failed_login_attempts: 0,
            last_failed_login_attempt: None,
            is_disabled: false,
            roles: Vec::new(),
        })
    }
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
        let pw = Password::new(raw_pw).unwrap();
        assert_ne!(pw.0, String::from(raw_pw));
    }

    #[test]
    pub fn password_verification_succeeds_when_same() {
        let raw_pw = "hunter2";
        let pw = Password::new(raw_pw).unwrap();
        let valid = pw.verify(b"hunter2");
        assert!(valid);
    }

    #[test]
    pub fn password_verification_fails_when_different() {
        let raw_pw = "hunter2";
        let pw = Password::new(raw_pw).unwrap();
        let valid = pw.verify(b"hunter123");
        assert!(!valid);
    }

    #[test]
    fn generate_admin_password_hash() {
        let password = b"admin";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

        println!("Admin password hash: {password_hash}");
        // Copy this hash to your migration
    }
}
