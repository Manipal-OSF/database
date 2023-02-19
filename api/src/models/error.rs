use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    AuthenticationError,
    ValidationError(String),
    ServerError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::AuthenticationError => (
                StatusCode::UNAUTHORIZED,
                "Authentication failed! Invalid credentials.".to_string(),
            ),
            Self::ValidationError(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            Self::ServerError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server Error | {message}"),
            ),
        };

        let body = Json(json!({ "message": message }));
        (status, body).into_response()
    }
}
