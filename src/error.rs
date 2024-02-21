use axum::{body::Body, http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use serde_json::to_string_pretty;
pub use tracing::{debug, error, info, warn};

pub type Result<T> = core::result::Result<T, ErrorStruct>;

#[derive(Serialize, Debug)]
pub enum ErrorType {
    InternalError,
}

#[derive(Serialize, Debug)]
pub struct ErrorStruct {
    name: ErrorType,
    title: String,
    description: String,
}

impl ErrorStruct {
    pub fn new(title: String, description: String) -> Self {
        Self {
            name: ErrorType::InternalError,
            title,
            description
        }
    }
}

impl IntoResponse for ErrorStruct {
    fn into_response(self) -> Response {
        error!("{self:?}");

        (StatusCode::INTERNAL_SERVER_ERROR, [("Content-Type", "application/json")], to_string_pretty(&self).unwrap()).into_response()
    }
}