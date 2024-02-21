use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;


pub fn data_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./data")))
}