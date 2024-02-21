use axum::{extract::Query, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;
use crate::{ErrorStruct, Result};

#[derive(Debug, Deserialize)]
pub struct TestError {
    error: bool,
}

pub async fn test_error(Query(params): Query<TestError>) -> Result<Json<Value>> {
    debug!("{params:?}");

    if params.error {
        return Err(ErrorStruct::new("Test Complete".to_string(), "false".to_string()));
    }

    let body = Json(json!({
        "result": true
    }));

    Ok(body)
}