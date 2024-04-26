use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, Query, State, Json};
use chrono::{Local, NaiveDateTime};
use serde::{Serialize, Deserialize};
use log::debug;
use sea_orm::Set;

use crate::{
    db::schemas::user, error::ApiResult, AppState, AvatarStyle, UserRank
};

#[derive(Serialize, Deserialize)]
pub struct UserHttpAnswer {
    pub name: String,
    #[serde(rename = "creationTime")]
    pub creation_time: NaiveDateTime,
    #[serde(rename = "lastLoginTime")]
    pub last_login_time: Option<NaiveDateTime>,
    pub version: i32,
    pub rank: UserRank,
    #[serde(rename = "avatarStyle")]
    pub avatar_style: AvatarStyle,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
    #[serde(rename = "commentCount")]
    pub comment_count: i32,
    #[serde(rename = "uploadedPostCount")]
    pub uploaded_post_count: i32,
    #[serde(rename = "favoritePostCount")]
    pub favorite_post_count: i32,
    #[serde(rename = "likedPostCount")]
    pub liked_post_count: i32,
    #[serde(rename = "dislikedPostCount")]
    pub disliked_post_count: i32,
    pub email: Option<String>,
}

// TODO! Rework all structs to use 'rename_all = "camelCase"'
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MicroUser {
    pub name: String,
    pub avatar_url: String,
}

#[derive(Debug, Deserialize)]
pub struct UserHttpQuery {
    #[serde(rename = "bump-login")]
    pub bump_login: bool,
}

impl Default for UserHttpQuery {
    fn default() -> Self {
        Self { bump_login: false }
    }
}

pub async fn get_user(
    Path(user): Path<String>,
    params: Option<Query<UserHttpQuery>>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<UserHttpAnswer>> {

    let mut raw_user = state.db.get_user_by_name(&user).await?;

    // Update last login time if needed P.S. Лишние операции... так то это всё true false нахуй не надо!
    let Query(params) = params.unwrap_or_default();
    if params.bump_login == true {
        raw_user = state.db.update_last_login_time(&raw_user.name).await?
    }

    Ok(Json(UserHttpAnswer {
        name: user,
        creation_time: raw_user.creation_time,
        last_login_time: raw_user.last_login_time,
        version: raw_user.version,
        rank: UserRank::from_str(&raw_user.rank).unwrap(),
        avatar_style: AvatarStyle::from_str(&raw_user.avatar_style).unwrap(),
        avatar_url: "data/avatarka.jpg".to_string(),    // TODO! Hardcoded shit!
        comment_count: 0,                               // TODO!
        uploaded_post_count: 0,                         // TODO!
        favorite_post_count: 0,                         // TODO!
        liked_post_count: 0,                            // TODO!
        disliked_post_count: 0,                         // TODO!
        email: raw_user.email,
    }))
}

#[derive(Deserialize, Debug)]
pub struct CreateUserHttpQuery {
    pub name: String,
    pub password: String,
    pub email: Option<String>,
    pub rank: Option<UserRank>,
    #[serde(rename = "avatarStyle")]
    pub avatar_style: Option<AvatarStyle>,
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(params): Json<CreateUserHttpQuery>, // ЭТА ЕБУЧАЯ ХУЙНЯ ДОЛЖНА БЫТЬ ПОСЛЕДНЕЙ, НАВОДИСЬ НА JSON И ПРОЧИТАЙ ПОСЛЕДНЮЮ СТРОКУ СПРАВКИ
) -> ApiResult<Json<UserHttpAnswer>> {
    debug!("Trying to create new user with credentials: {params:?}");
    let form_data = user::ActiveModel {
        name: Set(params.name.clone()),            // TODO!
        password_hash: Set("aaa".to_string()),     // Переделать под Default
        password_salt: Set(Some("aaa".to_string())),     // За пример взять UserToken
        email: Set(params.email.clone()),
        rank: Set(UserRank::Administrator.to_string()),
        creation_time: Set(Local::now().naive_local().to_owned()),
        avatar_style: Set(AvatarStyle::Gravatar.to_string()),
        ..Default::default()
    };
    let created_user = state.db.create_user(form_data).await?;

    let raw_user = state.db.get_user_by_id(created_user.id.unwrap() as u64).await?;
    Ok(Json(UserHttpAnswer {
        name: raw_user.name,
        creation_time: raw_user.creation_time,
        last_login_time: raw_user.last_login_time,
        version: raw_user.version,
        rank: UserRank::from_str(&raw_user.rank).unwrap(),
        avatar_style: AvatarStyle::from_str(&raw_user.avatar_style).unwrap(),
        avatar_url: "data/avatarka.jpg".to_string(),    // TODO! Hardcoded shit!
        comment_count: 0,                               // TODO!
        uploaded_post_count: 0,                         // TODO!
        favorite_post_count: 0,                         // TODO!
        liked_post_count: 0,                            // TODO!
        disliked_post_count: 0,                         // TODO!
        email: raw_user.email,
    }))
}