use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum RestError {
    Invalid(String),
    Internal(anyhow::Error),
}

impl IntoResponse for RestError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            RestError::Invalid(msg) => (StatusCode::BAD_REQUEST, msg),
            RestError::Internal(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for RestError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err.into())
    }
}
