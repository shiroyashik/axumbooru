use std::sync::Arc;

use axum::{extract::{Path, State}, Json};
use chrono::{DateTime, Local, Months, NaiveDateTime};
use serde::{Deserialize, Serialize};
use service::{UserQuery, UserTokenMutation, UserTokenQuery};
use log::debug;
use uuid::Uuid;

use crate::{AppState, Result};

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
// и будет браться от сюда. TODO! Брать значение offset'a из конфига, а не хардкод!
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
) -> Result<Json<UserTokenHttpResponse>> {
    debug!("Trying to create new user-token for '{user}' with params: {params:?}");
    let user = UserQuery::find_user_by_name(&state.db, &user).await.expect("DB error!").expect("Can't found user!");
    let form_data = UserTokenMutation {
        user_id: Some(user.id),
        token: Some(Uuid::new_v4().to_string()),
        // Ооо! Пресвятой колхоз!
        note: Some(params.note.unwrap_or(CreateUserTokenHttpQuery::default().note.unwrap())),
        enabled: Some(params.enabled.unwrap_or(CreateUserTokenHttpQuery::default().enabled.unwrap())),
        // Ооо! Пресвятой колхоз со временем!
        expiration_time: Some(params.expiration_time.unwrap_or(CreateUserTokenHttpQuery::default().expiration_time.unwrap()).naive_utc()),
        ..Default::default()
    };
    UserTokenMutation::create_token(&state.db, form_data.clone()).await.expect("DB error!");
    let raw_token = UserTokenQuery::find_token(&state.db, &form_data.token.unwrap()).await.expect("DB error!").expect("Can't found created user-token!");
    Ok(Json(UserTokenHttpResponse {
        user: MicroUser { name: user.name, avatar_url: "data/avatarka.jpg".to_string() },
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUserTokensHttpResponse {
    pub results: Vec<UserTokenHttpResponse>,
}

pub async fn list_usertokens( // GET
    Path(user): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ListUserTokensHttpResponse>> {
    let user = UserQuery::find_user_by_name(&state.db, &user).await.expect("DB error!").expect("Can't found user!");
    let raw_tokens = UserTokenQuery::find_list_user_tokens_by_user_id(&state.db, user.id).await.expect("DB error!");
    let miniuser = MicroUser {
        name: user.name,
        avatar_url: "data/avatarka.jpg".to_string(), // TODO!
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