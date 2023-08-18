use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;


pub enum CustomError {
    BadRequest,
    TodoNotFound,
    TodoDuplicate,
    InternalServerError,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            ),
            Self::BadRequest=> (StatusCode::BAD_REQUEST, "Bad Request"),
            Self::TodoNotFound => (StatusCode::NOT_FOUND, "Todo Not Found"),
            Self::TodoDuplicate => (StatusCode::CONFLICT, "Todo Duplicated")
        };
        (status, Json(json!({"error": error_message}))).into_response()
    }
}
