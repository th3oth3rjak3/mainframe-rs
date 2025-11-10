use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

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

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::internal(err.to_string())
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
