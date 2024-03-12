use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

use crate::config::{AvatarStyle, UserRank};
use crate::{AppState, Result};
use service::{UserQuery, UserMutation};

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

pub async fn get_user_by_id(
    Path(user): Path<String>,
    params: Option<Query<UserHttpQuery>>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserHttpAnswer>> {

    let mut raw_user = UserQuery::find_user_by_name(&state.db, &user).await.expect("Database error!").expect("User not found!");

    // Update last login time if needed P.S. Лишние операции... так то это всё true false нахуй не надо!
    let Query(params) = params.unwrap_or_default();
    if params.bump_login == true {
        raw_user = UserMutation::update_last_login_time_by_name(&state.db, &raw_user.name).await.expect("Database error!")
    }

    let user: UserHttpAnswer = UserHttpAnswer {
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
    };
    Ok(Json(user))
}