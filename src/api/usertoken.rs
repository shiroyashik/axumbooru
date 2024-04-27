use std::sync::Arc;

use axum::{extract::{Path, State}, Json};
use chrono::{DateTime, Local, Months, NaiveDateTime};
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use log::debug;
use uuid::Uuid;

use crate::{
    db::schemas::user_token, error::{ApiError, ApiResult}, AppState
};

use super::user::MicroUser;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTokenHttpResponse {
    pub user: MicroUser,
    pub token: String,
    pub note: Option<String>,
    pub enabled: bool,
    pub expiration_time: Option<NaiveDateTime>,
    pub creation_time: NaiveDateTime,
    pub last_edit_time: Option<NaiveDateTime>,
    pub last_usage_time: Option<NaiveDateTime>,
    pub version: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserTokenHttpQuery {
    pub enabled: Option<bool>,
    pub note: Option<String>,
    pub expiration_time: Option<DateTime<Local>>,
}

// Внимание! Строчки дальше отвечают за состояния по умолчанию при создании токена
// проблема в том что при логине на фронтенде "expiration_time" не указывается
// и будет браться от сюда. TODO: Брать значение offset'a из конфига, а не хардкод!
impl Default for CreateUserTokenHttpQuery {
    fn default() -> Self {
        Self {
            enabled: Some(true),
            note: Some("Undefined due creation".to_string()),
            expiration_time: Some(Local::now().checked_add_months(Months::new(12)).unwrap())
        }
    }
}

pub async fn create_usertoken( // POST
    Path(user): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(params): Json<CreateUserTokenHttpQuery>, // ЭТА ЕБУЧАЯ ХУЙНЯ ДОЛЖНА БЫТЬ ПОСЛЕДНЕЙ, НАВОДИСЬ НА JSON И ПРОЧИТАЙ ПОСЛЕДНЮЮ СТРОКУ СПРАВКИ
) -> ApiResult<Json<UserTokenHttpResponse>> {
    debug!("Trying to create new user-token for '{user}' with params: {params:?}");
    let user = state.db.get_user_by_name(&user).await?;
    let form_data = user_token::ActiveModel {
        user_id: Set(user.id),
        token: Set(Uuid::new_v4().to_string()),
        // Ооо! Пресвятой колхоз!
        note: Set(Some(params.note.unwrap_or(CreateUserTokenHttpQuery::default().note.unwrap()))),
        enabled: Set(params.enabled.unwrap_or(CreateUserTokenHttpQuery::default().enabled.unwrap())),
        // Ооо! Пресвятой колхоз со временем!
        expiration_time: Set(Some(params.expiration_time.unwrap_or(CreateUserTokenHttpQuery::default().expiration_time.unwrap()).naive_utc())),
        ..Default::default()
    };
    debug!("{form_data:?}");
    state.db.create_user_token(form_data.clone()).await?;
    let raw_token = state.db.get_user_token(&form_data.token.unwrap()).await?;
    Ok(Json(UserTokenHttpResponse {
        user: MicroUser { name: user.name, avatar_url: "data/avatarka.jpg".to_string() }, // FIXME: hardcoded
        token: raw_token.token,
        note: raw_token.note,
        enabled: raw_token.enabled,
        expiration_time: raw_token.expiration_time,
        creation_time: raw_token.creation_time,
        last_edit_time: raw_token.last_edit_time,
        last_usage_time: raw_token.last_usage_time,
        version: raw_token.version,
    }))
}

pub async fn delete_usertoken(
    Path((user, token)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<&'static str> {
    if state.db.get_user_by_name(&user).await?.id == state.db.get_user_token(&token).await?.user_id {
        state.db.delete_user_token(&token).await?;
        debug!("Token {token} deleted!")
    } else {
        return Err(ApiError::DeleteToken(crate::db::errors::DeleteUserTokenError::TokenUserIdDontMatch));
    }
    // Output
    Ok("{}")
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUserTokensHttpResponse {
    pub results: Vec<UserTokenHttpResponse>,
}

pub async fn list_usertokens( // GET
    Path(user): Path<String>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ListUserTokensHttpResponse>> {
    let user = state.db.get_user_by_name(&user).await?;
    let raw_tokens = state.db.get_user_tokens_by_user_id(user.id as u64).await?;
    let miniuser = MicroUser {
        name: user.name,
        avatar_url: "data/avatarka.jpg".to_string(), // FIXME: hardcoded
    };
    let mut prepared_tokens: Vec<UserTokenHttpResponse> = Vec::new();
    for model in raw_tokens.iter() {
        prepared_tokens.push(UserTokenHttpResponse {
            user: miniuser.clone(),
            token: model.token.to_owned(),
            note: model.note.to_owned(),
            enabled: model.enabled,
            expiration_time: model.expiration_time,
            creation_time: model.creation_time,
            last_edit_time: model.last_edit_time,
            last_usage_time: model.last_usage_time,
            version: model.version,
        })
    }
    Ok(Json(ListUserTokensHttpResponse {
        results: prepared_tokens,
    }))
}