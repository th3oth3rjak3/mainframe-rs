//! Defines the error handling architecture for the application.
//!
//! This module implements a three-tiered error system to create a clean separation
//! between the data access, business logic, and web layers.
//!
//! - `RepositoryError`: For failures in the data access layer.
//! - `ServiceError`: For failures in the business logic (service) layer.
//! - `ApiError`: For representing errors at the HTTP boundary, responsible for
//!   generating appropriate log messages and client-facing responses.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::fmt::Debug;
use thiserror::Error;

/// The structured, public-facing error response sent to API clients.
///
/// This struct is serialized to JSON and provides a consistent error format
/// for all client-facing errors, without leaking internal implementation details.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// The primary error type for the Axum HTTP boundary.
///
/// This enum acts as an "anti-corruption layer," decoupling the web framework
/// from the application's internal error types (`ServiceError`). Its sole
/// responsibility is to be converted into an HTTP `Response`.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("NotFound: {entity} with {property} {value} not found")]
    NotFound {
        entity: &'static str,
        property: &'static str,
        value: String,
    },
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Unauthorized")]
    Unauthorized {
        /// The internal reason for the failure, used for logging.
        reason: String,
    },
    #[error("Forbidden")]
    Forbidden {
        /// The internal reason for the failure, used for logging.
        reason: String,
    },
    #[error("An internal server error occurred")]
    Internal(#[from] anyhow::Error),
}

/// Converts a `ServiceError` into the appropriate `ApiError`.
///
/// This implementation serves as the bridge between the application's business
/// logic and its web layer, allowing for seamless error propagation with the `?`
/// operator in Axum handlers.
impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::Unauthorized(reason) => Self::Unauthorized { reason },
            ServiceError::AccountLocked | ServiceError::InvalidUsernameOrPassword => {
                Self::Unauthorized {
                    reason: err.to_string(),
                }
            }
            ServiceError::Forbidden(reason) => Self::Forbidden { reason },
            ServiceError::BadRequest(msg) => Self::BadRequest(msg),
            ServiceError::Repository(repo_err) => match repo_err {
                RepositoryError::NotFound {
                    entity,
                    property,
                    value,
                } => Self::NotFound {
                    entity,
                    property,
                    value: value.to_string(),
                },
                RepositoryError::Database(e) => Self::Internal(e.into()),
                RepositoryError::ArgumentOutOfRange { .. } => {
                    Self::BadRequest(format!("bad request: {}", repo_err.to_string()))
                }
            },
            ServiceError::NotFound {
                entity,
                property,
                value,
            } => Self::NotFound {
                entity,
                property,
                value,
            },
            ServiceError::Internal(e) => Self::Internal(e),
        }
    }
}

/// Converts an `ApiError` into a user-facing HTTP `Response`.
///
/// This is the single point of truth for all error handling at the application
/// boundary. It is responsible for:
/// 1.  Mapping the `ApiError` variant to the correct `StatusCode`.
/// 2.  Performing structured logging with `tracing`.
/// 3.  Ensuring sensitive details are logged but not sent to the client.
/// 4.  Serializing a public-facing `ErrorResponse` as the JSON response body.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, log_as_error) = match &self {
            Self::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, true),
            Self::NotFound { .. } => (StatusCode::NOT_FOUND, false),
            Self::BadRequest(_) => (StatusCode::BAD_REQUEST, false),
            Self::Unauthorized { .. } => (StatusCode::UNAUTHORIZED, false),
            Self::Forbidden { .. } => (StatusCode::FORBIDDEN, false),
        };

        if log_as_error {
            // For 5xx errors, log the full `Debug` representation, including the source chain.
            tracing::error!(error = ?self, "Request failed with a server error");
        } else {
            // For 4xx errors, log the specific reason for auth failures, or the
            // general error for other client issues.
            match &self {
                Self::Unauthorized { reason } => {
                    tracing::warn!(reason = %reason, "Unauthorized access attempt");
                }
                Self::Forbidden { reason } => {
                    tracing::warn!(reason = %reason, "Forbidden access attempt");
                }
                _ => {
                    tracing::info!(error = %self, "Request failed with a client error");
                }
            }
        }

        // Create the public-facing JSON response, ensuring no sensitive reasons are included.
        let public_error_message = match self {
            Self::Internal(_) => "An internal server error occurred".to_string(),
            Self::Unauthorized { .. } => "Unauthorized".to_string(),
            Self::Forbidden { .. } => "Forbidden".to_string(),
            _ => self.to_string(),
        };

        let body = Json(ErrorResponse {
            error: public_error_message,
        });

        (status, body).into_response()
    }
}

/// Represents failures within the application's service (business logic) layer.
///
/// This error enum should have no knowledge of HTTP-specific concepts. It is
/// used to represent business rule violations, authorization failures, and to
/// wrap errors from the underlying repository layer.
#[derive(Debug, Error)]
#[allow(unused)]
pub enum ServiceError {
    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("invalid username or password")]
    InvalidUsernameOrPassword,

    #[error("account locked due to repeated, failed login attempts")]
    AccountLocked,

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error("{entity} with {property} {value} not found")]
    NotFound {
        entity: &'static str,
        property: &'static str,
        value: String,
    },

    #[error("internal service error")]
    Internal(#[from] anyhow::Error),
}

/// Represents errors that occur within the data access (repository) layer.
///
/// This is the most foundational error type, concerned only with the state
/// of data persistence, such as database connection issues or missing records.
#[derive(Debug, Error)]
pub enum RepositoryError {
    /// Low‑level SQLx errors (connection failures, constraint violations, …).
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    /// A record was not found in the database.
    #[error("{entity} with {property} {value} not found")]
    NotFound {
        entity: &'static str,
        property: &'static str,
        value: String,
    },

    /// an argument passed to a repository method is outside
    /// the acceptable range (e.g., negative page number, zero page size).
    #[error("argument out of range: {field} with value `{value}` is not allowed")]
    ArgumentOutOfRange { field: &'static str, value: String },
}
