use crate::{
    config::{self, Privileges},
    AppState, Config,
};
use axum::{extract::State, Json};
use chrono::prelude::*;
use config::UserRank;
use serde::{Deserialize, Serialize};
use service::PostQuery;
use std::sync::Arc;
use log::debug;

#[derive(Serialize, Deserialize)]
pub struct InfoAnswer {
    #[serde(rename = "postCount")]
    post_count: u64,
    #[serde(rename = "diskUsage")]
    disk_usage: u64,
    #[serde(rename = "serverTime")]
    server_time: chrono::NaiveDateTime,
    #[serde(rename = "config")]
    config: FrontendConfig,
    #[serde(rename = "featuredPost")]
    featured_post: Option<()>, // Temporarily removed
    #[serde(rename = "featuringUser")]
    featuring_user: Option<()>, // Temporarily removed
    #[serde(rename = "featuringTime")]
    featuring_time: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct FrontendConfig {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "userNameRegex")]
    user_name_regex: String,
    #[serde(rename = "passwordRegex")]
    password_regex: String,
    #[serde(rename = "tagNameRegex")]
    tag_name_regex: String,
    #[serde(rename = "tagCategoryNameRegex")]
    tag_category_name_regex: String,
    #[serde(rename = "defaultUserRank")]
    default_user_rank: UserRank,
    #[serde(rename = "enableSafety")]
    enable_safety: bool,
    #[serde(rename = "contactEmail")]
    contact_email: String,
    #[serde(rename = "canSendMails")]
    can_send_mails: bool,
    #[serde(rename = "privileges")]
    privileges: Privileges,
}

impl FrontendConfig {
    pub async fn from_config(config: Config) -> Self {
        Self {
            name: config.name,
            user_name_regex: config.user_name_regex,
            password_regex: config.password_regex,
            tag_name_regex: config.tag_name_regex,
            tag_category_name_regex: config.tag_category_name_regex,
            default_user_rank: config.default_rank,
            enable_safety: config.enable_safety,
            contact_email: config.contact_email,
            can_send_mails: config.smtp.enabled,
            privileges: config.privileges,
        }
    }
}

// #[derive(Serialize, Deserialize)]
// pub struct FeaturedPost {
//     #[serde(rename = "id")]
//     id: i64,
//     #[serde(rename = "version")]
//     version: i64,
//     #[serde(rename = "creationTime")]
//     creation_time: chrono::NaiveDateTime,
//     #[serde(rename = "lastEditTime")]
//     last_edit_time: Option<chrono::NaiveDateTime>,
//     #[serde(rename = "safety")]
//     safety: String,
//     #[serde(rename = "source")]
//     source: Option<serde_json::Value>,
//     #[serde(rename = "type")]
//     featured_post_type: String,
//     #[serde(rename = "mimeType")]
//     mime_type: String,
//     #[serde(rename = "checksum")]
//     checksum: String,
//     #[serde(rename = "checksumMD5")]
//     checksum_md5: String,
//     #[serde(rename = "fileSize")]
//     file_size: i64,
//     #[serde(rename = "canvasWidth")]
//     canvas_width: i64,
//     #[serde(rename = "canvasHeight")]
//     canvas_height: i64,
//     #[serde(rename = "contentUrl")]
//     content_url: String,
//     #[serde(rename = "thumbnailUrl")]
//     thumbnail_url: String,
//     #[serde(rename = "flags")]
//     flags: Vec<String>,
//     #[serde(rename = "tags")]
//     tags: Vec<Tag>,
//     #[serde(rename = "relations")]
//     relations: Vec<Option<serde_json::Value>>,
//     #[serde(rename = "user")]
//     user: User,
//     #[serde(rename = "score")]
//     score: i64,
//     #[serde(rename = "ownScore")]
//     own_score: i64,
//     #[serde(rename = "ownFavorite")]
//     own_favorite: bool,
//     #[serde(rename = "tagCount")]
//     tag_count: i64,
//     #[serde(rename = "favoriteCount")]
//     favorite_count: i64,
//     #[serde(rename = "commentCount")]
//     comment_count: i64,
//     #[serde(rename = "noteCount")]
//     note_count: i64,
//     #[serde(rename = "relationCount")]
//     relation_count: i64,
//     #[serde(rename = "featureCount")]
//     feature_count: i64,
//     #[serde(rename = "lastFeatureTime")]
//     last_feature_time: Option<chrono::NaiveDateTime>,
//     #[serde(rename = "favoritedBy")]
//     favorited_by: Vec<Option<serde_json::Value>>,
//     #[serde(rename = "hasCustomThumbnail")]
//     has_custom_thumbnail: bool,
//     #[serde(rename = "notes")]
//     notes: Vec<Option<serde_json::Value>>,
//     #[serde(rename = "comments")]
//     comments: Vec<Comment>,
//     #[serde(rename = "pools")]
//     pools: Vec<Option<serde_json::Value>>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Comment {
//     #[serde(rename = "id")]
//     id: i64,
//     #[serde(rename = "user")]
//     user: User,
//     #[serde(rename = "postId")]
//     post_id: i64,
//     #[serde(rename = "version")]
//     version: i64,
//     #[serde(rename = "text")]
//     text: String,
//     #[serde(rename = "creationTime")]
//     creation_time: chrono::NaiveDateTime,
//     #[serde(rename = "lastEditTime")]
//     last_edit_time: Option<chrono::NaiveDateTime>,
//     #[serde(rename = "score")]
//     score: i64,
//     #[serde(rename = "ownScore")]
//     own_score: i64,
// }

// #[derive(Serialize, Deserialize)]
// pub struct User {
//     #[serde(rename = "name")]
//     name: String,
//     #[serde(rename = "avatarUrl")]
//     avatar_url: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Tag {
//     #[serde(rename = "names")]
//     names: Vec<String>,
//     #[serde(rename = "category")]
//     category: String,
//     #[serde(rename = "usages")]
//     usages: i64,
// }

// #[derive(Serialize, Deserialize)]
// pub struct FeaturingUser {
//     #[serde(rename = "name")]
//     name: String,
//     #[serde(rename = "creationTime")]
//     creation_time: chrono::NaiveDateTime,
//     #[serde(rename = "lastLoginTime")]
//     last_login_time: Option<chrono::NaiveDateTime>,
//     #[serde(rename = "version")]
//     version: i64,
//     #[serde(rename = "rank")]
//     rank: UserRank,
//     #[serde(rename = "avatarStyle")]
//     avatar_style: String,
//     #[serde(rename = "avatarUrl")]
//     avatar_url: String,
//     #[serde(rename = "commentCount")]
//     comment_count: i64,
//     #[serde(rename = "uploadedPostCount")]
//     uploaded_post_count: i64,
//     #[serde(rename = "favoritePostCount")]
//     favorite_post_count: i64,
//     #[serde(rename = "likedPostCount")]
//     liked_post_count: bool,
//     #[serde(rename = "dislikedPostCount")]
//     disliked_post_count: bool,
//     #[serde(rename = "email")]
//     email: bool,
// }

pub async fn server_info(State(state): State<Arc<AppState>>) -> Json<InfoAnswer> {
    debug!("called");

    let info = InfoAnswer {
        post_count: PostQuery::count_posts(&state.db).await.unwrap(),
        disk_usage: crate::api::data::get_folder_size("./data").unwrap(),
        server_time: Local::now().naive_local(),
        config: FrontendConfig::from_config(state.config.clone()).await,
        featured_post: None,
        featuring_user: None,
        featuring_time: None,
    };

    Json(info)
}
