use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[allow(unused)]
#[derive(Responder)]
pub enum ApiError {
    #[response(status = 404, content_type = "json")]
    NotFound(Json<ErrorResponse>),

    #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorResponse>),

    #[response(status = 500, content_type = "json")]
    Internal(Json<ErrorResponse>),
}

impl ApiError {
    pub fn internal(msg: impl Into<String>) -> Self {
        ApiError::Internal(Json(ErrorResponse { error: msg.into() }))
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        ApiError::internal(value.to_string())
    }
}
