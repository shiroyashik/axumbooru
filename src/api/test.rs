use std::sync::Arc;

use crate::{error::*, AppState, RequireAuth};
use axum::{extract::{Query, State}, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use log::debug;

#[derive(Debug, Deserialize)]
pub struct Test {
    error: Option<bool>,
    token: Option<String>,
}

pub async fn test(
    Query(params): Query<Test>,
    auth: RequireAuth,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Value>> {
    // TODO: По плану в будущем это будет диагностикой
    // if params.is_some_and(|x| x.error) {
        // return Err(ErrorStruct::new(
        //     "Test Complete".to_string(),
        //     "false".to_string(),
        // ));
    // }
    debug!("Get: {params:?}");
    if params.error.is_none() && params.token.is_none() {
        let state = state.uploads.lock().expect("Uploads poisoned");
        let body = Json(json!({"result": true, "auth": auth.is_some(), "uploads": format!("{:?}", state.keys())}));
        
        debug!("{body:?}");
        return Ok(body);
    }
    if params.error.unwrap_or(false) {
        // return Err(ErrorStruct::new( TODO: Я отказался от использования прошлого error так что надо переделать
        //     "Test Complete".to_string(),
        //     "false".to_string(),
        // ));
        return Err(ApiError::Test(TestError::ItsJustForTest));
    }
    let token_raw = params.token.as_ref().unwrap();
    let state = state.uploads.lock().expect("Uploads poisoned");
    let token_info = state.get(token_raw);
    if token_info.is_none() {
        // return Err(ErrorStruct::new(
        //     "Test Complete".to_string(),
        //     "false".to_string(),
        // ));
        return Err(ApiError::Test(TestError::ItsJustForTest));
    }
    let body = Json(json!({"info": format!("{token_info:?}")}));

    debug!("{body:?}");
    Ok(body)
}

pub async fn newtest(
) -> ApiResult<Json<Value>> {
    return Err(ApiError::Test(TestError::ItsJustForTest));
}

pub async fn newtest2(
) -> ApiResult<Json<Value>> {
    return Err(ApiError::Test(TestError::SecondEntry));
}