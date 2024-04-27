use axum::{http::StatusCode, response::{IntoResponse, Response}};
use log::error;
use serde_json::json;

use crate::db::errors::{DatabaseError, DeleteUserTokenError, GetUserError};

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Test(#[from] TestError),
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error(transparent)]
    GetUser(#[from] GetUserError),
    #[error(transparent)]
    DeleteToken(#[from] DeleteUserTokenError),
}

#[derive(thiserror::Error, Debug)]
pub enum TestError {
    #[error("Its Just For Test")]
    ItsJustForTest, // TODO: Это только на время разработки
    #[error("Second error what can be")]
    SecondEntry, // TODO: И это в том числе!
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        error!("Error on request: {self}");
        let description = self.to_string();
        match self {
            ApiError::Test(TestError::ItsJustForTest) => internal_server_error("InternalError", "ItsJustForTest", &description),
            ApiError::Test(TestError::SecondEntry) => internal_server_error("InternalError", "SecondEntry", &description),
            ApiError::Database(_) => internal_server_error("InternalError", &description, &description),
            ApiError::GetUser(_) => internal_server_error("InternalError", &description, &description),
            ApiError::DeleteToken(DeleteUserTokenError::DatabaseError(_)) => internal_server_error("InternalError", &description, &description),
            ApiError::DeleteToken(DeleteUserTokenError::TokenNotFound { .. }) => internal_server_error("InternalError", &description, &description),
            ApiError::DeleteToken(DeleteUserTokenError::TokenUserIdDontMatch) => method_not_allowed(),
        }
    }
}

fn internal_server_error(name: &str, title: &str, description: &str) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [("Content-Type", "application/json")],
        json!({
            "name": name,
            "title": title,
            "description": description, 
        }).to_string(),
    ).into_response()
}

fn method_not_allowed() -> Response {
    (
        StatusCode::METHOD_NOT_ALLOWED,
    ).into_response()
}