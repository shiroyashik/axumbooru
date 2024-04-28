use std::sync::Arc;

use crate::{error::*, AppState, RequireAuth};
use axum::{extract::State, Json};
use serde_json::{json, Value};
use log::debug;

pub async fn test(
    auth: RequireAuth,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Value>> {
    // TODO: Может в будущем это будет диагностикой

    let state = state.uploads.lock().expect("Uploads poisoned").vec();
    let body = Json(json!({"auth": format!("{auth:?}"), "uploads": format!("{:?}", state)}));
    
    debug!("{body:?}");
    return Ok(body);

}

pub async fn newtest(
) -> ApiResult<Json<Value>> {
    return Err(ApiError::Test(TestError::ItsJustForTest));
}

pub async fn newtest2(
) -> ApiResult<Json<Value>> {
    return Err(ApiError::Test(TestError::SecondEntry));
}