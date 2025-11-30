use std::fmt::Display;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.error)
    }
}

#[derive(Debug)]
pub enum ApiError {
    NotFound(ErrorResponse),
    BadRequest(ErrorResponse),
    Internal(ErrorResponse),
    Unauthorized(ErrorResponse),
    Forbidden(ErrorResponse),
}

impl ApiError {
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(ErrorResponse { error: msg.into() })
    }

    pub fn not_found() -> Self {
        Self::NotFound(ErrorResponse {
            error: "Not Found".into(),
        })
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(ErrorResponse { error: msg.into() })
    }

    pub fn unauthorized() -> Self {
        Self::Unauthorized(ErrorResponse {
            error: "Unauthorized".into(),
        })
    }

    pub fn forbidden() -> Self {
        Self::Forbidden(ErrorResponse {
            error: "Forbidden".into(),
        })
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(r)
            | Self::BadRequest(r)
            | Self::Internal(r)
            | Self::Unauthorized(r)
            | Self::Forbidden(r) => f.write_str(&r.error),
        }
    }
}

impl From<RepositoryError> for ApiError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::Database(e) => {
                tracing::error!("Database error: {e}");
                Self::internal("Database error")
            }
            RepositoryError::Unauthorized => Self::unauthorized(),
            RepositoryError::NotFound => Self::not_found(),
            RepositoryError::Validation(msg) => Self::bad_request(msg),
            RepositoryError::Other(msg) => {
                tracing::error!("Unexpected error: {msg}");
                Self::internal(msg)
            }
        }
    }
}

// Convert to Axum response
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            Self::NotFound(e) => (StatusCode::NOT_FOUND, e.to_string()),
            Self::BadRequest(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            Self::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::Unauthorized(e) => (StatusCode::UNAUTHORIZED, e.to_string()),
            Self::Forbidden(e) => (StatusCode::FORBIDDEN, e.to_string()),
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

    #[error("not authorized")]
    Unauthorized,

    #[error("validation error: {0}")]
    Validation(String),

    #[error("other error: {0}")]
    Other(String),
}
