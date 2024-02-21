use axum::{extract::State, Json};
use serde::{Serialize, Deserialize};
use tracing::debug;
use std::{collections::HashMap, sync::Arc};
use crate::{config::{self, Privileges}, AppState, ErrorStruct, Result};
use chrono::prelude::*;
use config::UserRank::{self, *};

#[derive(Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "postCount")]
    post_count: i64,

    #[serde(rename = "diskUsage")]
    disk_usage: i64,

    #[serde(rename = "serverTime")]
    server_time: chrono::NaiveDateTime,

    #[serde(rename = "config")]
    config: Config,

    #[serde(rename = "featuredPost")]
    featured_post: Option<FeaturedPost>,

    #[serde(rename = "featuringUser")]
    featuring_user: Option<FeaturingUser>,

    #[serde(rename = "featuringTime")]
    featuring_time: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
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

    #[serde(rename = "UserRank")]
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

#[derive(Serialize, Deserialize)]
pub struct FeaturedPost {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "version")]
    version: i64,

    #[serde(rename = "creationTime")]
    creation_time: chrono::NaiveDateTime,

    #[serde(rename = "lastEditTime")]
    last_edit_time: Option<chrono::NaiveDateTime>,

    #[serde(rename = "safety")]
    safety: String,

    #[serde(rename = "source")]
    source: Option<serde_json::Value>,

    #[serde(rename = "type")]
    featured_post_type: String,

    #[serde(rename = "mimeType")]
    mime_type: String,

    #[serde(rename = "checksum")]
    checksum: String,

    #[serde(rename = "checksumMD5")]
    checksum_md5: String,

    #[serde(rename = "fileSize")]
    file_size: i64,

    #[serde(rename = "canvasWidth")]
    canvas_width: i64,

    #[serde(rename = "canvasHeight")]
    canvas_height: i64,

    #[serde(rename = "contentUrl")]
    content_url: String,

    #[serde(rename = "thumbnailUrl")]
    thumbnail_url: String,

    #[serde(rename = "flags")]
    flags: Vec<String>,

    #[serde(rename = "tags")]
    tags: Vec<Tag>,

    #[serde(rename = "relations")]
    relations: Vec<Option<serde_json::Value>>,

    #[serde(rename = "user")]
    user: User,

    #[serde(rename = "score")]
    score: i64,

    #[serde(rename = "ownScore")]
    own_score: i64,

    #[serde(rename = "ownFavorite")]
    own_favorite: bool,

    #[serde(rename = "tagCount")]
    tag_count: i64,

    #[serde(rename = "favoriteCount")]
    favorite_count: i64,

    #[serde(rename = "commentCount")]
    comment_count: i64,

    #[serde(rename = "noteCount")]
    note_count: i64,

    #[serde(rename = "relationCount")]
    relation_count: i64,

    #[serde(rename = "featureCount")]
    feature_count: i64,

    #[serde(rename = "lastFeatureTime")]
    last_feature_time: Option<chrono::NaiveDateTime>,

    #[serde(rename = "favoritedBy")]
    favorited_by: Vec<Option<serde_json::Value>>,

    #[serde(rename = "hasCustomThumbnail")]
    has_custom_thumbnail: bool,

    #[serde(rename = "notes")]
    notes: Vec<Option<serde_json::Value>>,

    #[serde(rename = "comments")]
    comments: Vec<Comment>,

    #[serde(rename = "pools")]
    pools: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "user")]
    user: User,

    #[serde(rename = "postId")]
    post_id: i64,

    #[serde(rename = "version")]
    version: i64,

    #[serde(rename = "text")]
    text: String,

    #[serde(rename = "creationTime")]
    creation_time: chrono::NaiveDateTime,

    #[serde(rename = "lastEditTime")]
    last_edit_time: Option<chrono::NaiveDateTime>,

    #[serde(rename = "score")]
    score: i64,

    #[serde(rename = "ownScore")]
    own_score: i64,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "avatarUrl")]
    avatar_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "names")]
    names: Vec<String>,

    #[serde(rename = "category")]
    category: String,

    #[serde(rename = "usages")]
    usages: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FeaturingUser {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "creationTime")]
    creation_time: chrono::NaiveDateTime,

    #[serde(rename = "lastLoginTime")]
    last_login_time: Option<chrono::NaiveDateTime>,

    #[serde(rename = "version")]
    version: i64,

    #[serde(rename = "rank")]
    rank: UserRank,

    #[serde(rename = "avatarStyle")]
    avatar_style: String,

    #[serde(rename = "avatarUrl")]
    avatar_url: String,

    #[serde(rename = "commentCount")]
    comment_count: i64,

    #[serde(rename = "uploadedPostCount")]
    uploaded_post_count: i64,

    #[serde(rename = "favoritePostCount")]
    favorite_post_count: i64,

    #[serde(rename = "likedPostCount")]
    liked_post_count: bool,

    #[serde(rename = "dislikedPostCount")]
    disliked_post_count: bool,

    #[serde(rename = "email")]
    email: bool,
}

pub async fn server_info(State(state): State<Arc<AppState>>,) -> Json<Response> {
    debug!("called");

    let info = Response {
        post_count: 0,
        disk_usage: 0,
        server_time: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
        config: Config {
            name: state.config.name.clone(),
            user_name_regex: "^[a-zA-Z0-9_-]{1,32}$".to_string(),
            password_regex: "^.{5,}$".to_string(),
            tag_name_regex: "^\\S+$".to_string(),
            tag_category_name_regex: "^[^\\s%+#/]+$".to_string(),
            default_user_rank: Regular,
            enable_safety: true,
            contact_email: "test@axum.local".to_string(),
            can_send_mails: false,
            privileges: config::Privileges {
                users_create_self: Anonymous,
                users_create_any: Anonymous,
                users_list: Anonymous,
                users_view: Anonymous,
                users_edit_any_name: Anonymous,
                users_edit_any_pass: Anonymous,
                users_edit_any_email: Anonymous,
                users_edit_any_avatar: Anonymous,
                users_edit_any_rank: Anonymous,
                users_edit_self_name: Anonymous,
                users_edit_self_pass: Anonymous,
                users_edit_self_email: Anonymous,
                users_edit_self_avatar: Anonymous,
                users_edit_self_rank: Anonymous,
                users_delete_any: Anonymous,
                users_delete_self: Anonymous,
                user_tokens_list_any: Anonymous,
                user_tokens_list_self: Anonymous,
                user_tokens_create_any: Anonymous,
                user_tokens_create_self: Anonymous,
                user_tokens_edit_any: Anonymous,
                user_tokens_edit_self: Anonymous,
                user_tokens_delete_any: Anonymous,
                user_tokens_delete_self: Anonymous,
                posts_create_anonymous: Anonymous,
                posts_create_identified: Anonymous,
                posts_list: Anonymous,
                posts_reverse_search: Anonymous,
                posts_view: Anonymous,
                posts_view_featured: Anonymous,
                posts_edit_content: Anonymous,
                posts_edit_flags: Anonymous,
                posts_edit_notes: Anonymous,
                posts_edit_relations: Anonymous,
                posts_edit_safety: Anonymous,
                posts_edit_source: Anonymous,
                posts_edit_tags: Anonymous,
                posts_edit_thumbnail: Anonymous,
                posts_feature: Anonymous,
                posts_delete: Anonymous,
                posts_score: Anonymous,
                posts_merge: Anonymous,
                posts_favorite: Anonymous,
                posts_bulk_edit_tags: Anonymous,
                posts_bulk_edit_safety: Anonymous,
                posts_bulk_edit_delete: Anonymous,
                tags_create: Anonymous,
                tags_edit_names: Anonymous,
                tags_edit_category: Anonymous,
                tags_edit_description: Anonymous,
                tags_edit_implications: Anonymous,
                tags_edit_suggestions: Anonymous,
                tags_list: Anonymous,
                tags_view: Anonymous,
                tags_merge: Anonymous,
                tags_delete: Anonymous,
                tag_categories_create: Anonymous,
                tag_categories_edit_name: Anonymous,
                tag_categories_edit_color: Anonymous,
                tag_categories_edit_order: Anonymous,
                tag_categories_list: Anonymous,
                tag_categories_view: Anonymous,
                tag_categories_delete: Anonymous,
                tag_categories_set_default: Anonymous,
                pools_create: Anonymous,
                pools_edit_names: Anonymous,
                pools_edit_category: Anonymous,
                pools_edit_description: Anonymous,
                pools_edit_posts: Anonymous,
                pools_list: Anonymous,
                pools_view: Anonymous,
                pools_merge: Anonymous,
                pools_delete: Anonymous,
                pool_categories_create: Anonymous,
                pool_categories_edit_name: Anonymous,
                pool_categories_edit_color: Anonymous,
                pool_categories_list: Anonymous,
                pool_categories_view: Anonymous,
                pool_categories_delete: Anonymous,
                pool_categories_set_default: Anonymous,
                comments_create: Anonymous,
                comments_delete_any: Anonymous,
                comments_delete_own: Anonymous,
                comments_edit_any: Anonymous,
                comments_edit_own: Anonymous,
                comments_list: Anonymous,
                comments_view: Anonymous,
                comments_score: Anonymous,
                snapshots_list: Anonymous,
                uploads_create: Anonymous,
                uploads_use_downloader: Anonymous,
            },
        },
        featured_post: None,
        featuring_user: None,
        featuring_time: None,
    };

    Json(info)
}