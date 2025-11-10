use std::fmt::Display;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug)]
pub enum ApiError {
    NotFound(ErrorResponse),
    BadRequest(ErrorResponse),
    Internal(ErrorResponse),
}

impl ApiError {
    pub fn internal(msg: impl Into<String>) -> Self {
        ApiError::Internal(ErrorResponse { error: msg.into() })
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        ApiError::NotFound(ErrorResponse { error: msg.into() })
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        ApiError::BadRequest(ErrorResponse { error: msg.into() })
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(r) => f.write_str(&r.error),
            Self::BadRequest(r) => f.write_str(&r.error),
            Self::Internal(r) => f.write_str(&r.error),
        }
    }
}

impl From<RepositoryError> for ApiError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::Database(e) => {
                tracing::error!("Database error: {e}");
                ApiError::internal("Database error")
            }
            RepositoryError::NotFound => ApiError::not_found("Not found"),
            RepositoryError::Validation(msg) => ApiError::bad_request(msg),
            RepositoryError::Other(msg) => {
                tracing::error!("Unexpected error: {msg}");
                ApiError::internal(msg)
            }
        }
    }
}

// Convert to Axum response
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            ApiError::NotFound(e) => (StatusCode::NOT_FOUND, serde_json::to_string(&e).unwrap()),
            ApiError::BadRequest(e) => {
                (StatusCode::BAD_REQUEST, serde_json::to_string(&e).unwrap())
            }
            ApiError::Internal(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::to_string(&e).unwrap(),
            ),
        };
        (status, body).into_response()
    }
}

#[derive(Debug, Error)]
#[allow(unused)]
pub enum RepositoryError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("not found")]
    NotFound,

    #[error("validation error: {0}")]
    Validation(String),

    #[error("other error: {0}")]
    Other(String),
}
