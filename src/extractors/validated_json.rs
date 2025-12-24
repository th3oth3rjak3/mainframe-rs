use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T: Validate>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ValidationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // First extract as JSON
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(ValidationError::JsonRejection)?;

        // Then validate
        value.validate().map_err(ValidationError::Validation)?;

        Ok(Self(value))
    }
}

// Custom error type for validation failures
#[derive(Debug)]
pub enum ValidationError {
    JsonRejection(JsonRejection),
    Validation(validator::ValidationErrors),
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        match self {
            Self::JsonRejection(rejection) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid JSON: {rejection}"),
            )
                .into_response(),
            Self::Validation(errors) => (
                StatusCode::BAD_REQUEST,
                format!("Validation failed: {errors}"),
            )
                .into_response(),
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsonRejection(rejection) => write!(f, "Invalid JSON: {rejection}"),
            Self::Validation(errors) => write!(f, "Validation failed: {errors}"),
        }
    }
}

impl std::error::Error for ValidationError {}
