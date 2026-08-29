use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

fn json_error(status: StatusCode, message: &str) -> Response<Body> {
    let body = Json(json!({ "error": message }));
    (status, body).into_response()
}

pub enum AppError {
    Auth,
    UserExists,
    UserRegistrationDisabled,
    Internal,
    BookNotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        match self {
            AppError::UserExists => {
                json_error(StatusCode::UNPROCESSABLE_ENTITY, "User already exists!")
            }
            AppError::UserRegistrationDisabled => {
                json_error(StatusCode::FORBIDDEN, "User registration disabled")
            }
            AppError::Internal => {
                json_error(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::BookNotFound => {
                json_error(StatusCode::NOT_FOUND, "Book not found to get progress")
            }
            AppError::Auth => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Authorisation failed" })),
            )
                .into_response(),
        }
    }
}
