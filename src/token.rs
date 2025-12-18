// src/token.rs
use crate::errors::ApiError;
use argon2::password_hash::rand_core::{OsRng, RngCore};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

/// Represents a parsed session token in the format "uuid:token"
#[derive(Debug, Clone)]
pub struct SessionToken {
    pub session_id: Uuid,
    pub raw_token: String,
}

impl SessionToken {
    /// Parse a cookie value in the format "uuid:token"
    pub fn parse(cookie_value: &str) -> Result<Self, ApiError> {
        let parts: Vec<&str> = cookie_value.split(':').collect();

        if parts.len() != 2 {
            return Err(ApiError::Unauthorized {
                reason: "invalid session token format".into(),
            });
        }

        let session_id = Uuid::parse_str(parts[0]).map_err(|_| ApiError::Unauthorized {
            reason: "invalid session id in token".into(),
        })?;

        let raw_token = parts[1].to_string();

        if raw_token.is_empty() {
            return Err(ApiError::Unauthorized {
                reason: "empty token in session".into(),
            });
        }

        Ok(Self {
            session_id,
            raw_token,
        })
    }

    /// Encode as "uuid:token" for cookie value
    pub fn encode(&self) -> String {
        format!("{}:{}", self.session_id, self.raw_token)
    }

    /// Hash the raw token using HMAC-SHA256 for storage in database
    pub fn hash_token(&self, hmac_key: &[u8]) -> Result<String, ApiError> {
        let mut mac = HmacSha256::new_from_slice(hmac_key)
            .map_err(|_| ApiError::Internal(anyhow::anyhow!("invalid HMAC key")))?;

        mac.update(self.raw_token.as_bytes());
        let result = mac.finalize();
        Ok(hex::encode(result.into_bytes()))
    }
}

/// Generate a new session token with random bytes
pub fn generate_session_token(session_id: Uuid) -> SessionToken {
    // Generate 32 bytes of random data for the token
    let token = get_token_bytes();

    SessionToken {
        session_id,
        raw_token: token,
    }
}

/// Verify a raw token against a stored hash
pub fn verify_token(raw_token: &str, stored_hash: &str, hmac_key: &[u8]) -> Result<bool, ApiError> {
    let mut mac = HmacSha256::new_from_slice(hmac_key)
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("invalid HMAC key")))?;

    mac.update(raw_token.as_bytes());
    let computed_hash = hex::encode(mac.finalize().into_bytes());

    Ok(computed_hash == stored_hash)
}

pub fn get_token_bytes() -> String {
    let mut token_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut token_bytes);
    hex::encode(token_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_token() {
        let uuid = Uuid::new_v4();
        let cookie_value = format!("{}:abc123def456", uuid);

        let token = SessionToken::parse(&cookie_value).unwrap();
        assert_eq!(token.session_id, uuid);
        assert_eq!(token.raw_token, "abc123def456");
    }

    #[test]
    fn test_encode_token() {
        let uuid = Uuid::new_v4();
        let token = SessionToken {
            session_id: uuid,
            raw_token: "abc123def456".to_string(),
        };

        let encoded = token.encode();
        assert_eq!(encoded, format!("{}:abc123def456", uuid));
    }

    #[test]
    fn test_hash_and_verify() {
        let hmac_key = b"test_key_32_bytes_long_exactly!!";
        let uuid = Uuid::new_v4();
        let token = SessionToken {
            session_id: uuid,
            raw_token: "test_token".to_string(),
        };

        let hash = token.hash_token(hmac_key).unwrap();
        let is_valid = verify_token("test_token", &hash, hmac_key).unwrap();
        assert!(is_valid);

        let is_invalid = verify_token("wrong_token", &hash, hmac_key).unwrap();
        assert!(!is_invalid);
    }
}
