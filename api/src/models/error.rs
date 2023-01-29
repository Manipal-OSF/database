use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    AuthenticationError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::AuthenticationError => (
                StatusCode::UNAUTHORIZED,
                "Authentication failed! Invalid credentials.".to_string(),
            ),
        };

        let body = Json(json!({ "message": message }));
        (status, body).into_response()
    }
}
