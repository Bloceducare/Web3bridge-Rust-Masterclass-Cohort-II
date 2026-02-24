use thiserror::Error;
use axum::{Json,  http::StatusCode, response::{IntoResponse}};
use serde_json::json;


#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
    #[error("Invalid Input, cannot be processed: {field} - {message}")]
    UnProcessableEntity { field: String, message: String },
    #[error("Environement Variable is missing: {0}")]
    MissingEnvironmentVarible(String),
    #[error("Failed to Parse: {0}")]
    ParsingError(String),
}


impl IntoResponse for AppError {
     fn into_response(self) -> axum::response::Response {
        let status = match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::UnProcessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::ParsingError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(json!({ "error": self.to_string() }))).into_response()
    }
}