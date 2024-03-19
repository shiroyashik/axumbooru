use crate::{ErrorStruct, RequireAuth, Result};
use axum::{extract::Query, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

#[derive(Debug, Deserialize)]
pub struct TestError {
    error: bool,
}

pub async fn test_error(
    params: Option<Query<TestError>>,
    auth: RequireAuth,
) -> Result<Json<Value>> {

    if params.is_some_and(|x| x.error) {
        return Err(ErrorStruct::new(
            "Test Complete".to_string(),
            "false".to_string(),
        ));
    }

    let body = Json(json!({"result": true, "auth": auth.has_authorization}));
    debug!("{body:?}");
    
    Ok(body)
}
