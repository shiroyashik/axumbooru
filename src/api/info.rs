// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "postCount")]
    post_count: i64,

    #[serde(rename = "diskUsage")]
    disk_usage: i64,

    #[serde(rename = "serverTime")]
    server_time: String,

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

    #[serde(rename = "defaultUserRank")]
    default_user_rank: DefaultUserRank,

    #[serde(rename = "enableSafety")]
    enable_safety: bool,

    #[serde(rename = "contactEmail")]
    contact_email: String,

    #[serde(rename = "canSendMails")]
    can_send_mails: bool,

    #[serde(rename = "privileges")]
    privileges: HashMap<String, DefaultUserRank>,
}

#[derive(Serialize, Deserialize)]
pub struct FeaturedPost {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "version")]
    version: i64,

    #[serde(rename = "creationTime")]
    creation_time: String,

    #[serde(rename = "lastEditTime")]
    last_edit_time: String,

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
    last_feature_time: String,

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
    creation_time: String,

    #[serde(rename = "lastEditTime")]
    last_edit_time: Option<serde_json::Value>,

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
    creation_time: String,

    #[serde(rename = "lastLoginTime")]
    last_login_time: String,

    #[serde(rename = "version")]
    version: i64,

    #[serde(rename = "rank")]
    rank: DefaultUserRank,

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

#[derive(Serialize, Deserialize)]
pub enum DefaultUserRank {
    #[serde(rename = "administrator")]
    Administrator,

    #[serde(rename = "moderator")]
    Moderator,

    #[serde(rename = "power")]
    Power,

    #[serde(rename = "regular")]
    Regular,

    #[serde(rename = "anonymous")]
    Anonymous,
}