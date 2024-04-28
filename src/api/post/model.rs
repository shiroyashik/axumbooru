use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct ListOfPostsAnswer {
    pub query: String,
    pub offset: u64,
    pub limit: u64,
    pub total: u64,
    pub results: Vec<MiniPost>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniPost {
    pub id: i32,
    pub thumbnail_url: String,
    pub r#type: String,
    pub safety: String,
    pub score: i32,
    pub favorite_count: i32,
    pub comment_count: i32,
    pub tags: Vec<()>, // Vec<Tag>
    pub version: i32,
}

impl MiniPost {
    pub fn from_model(
            model: &crate::db::schemas::post::Model,
            thumbnail_url: String,
            score: i32,
            favorite_count: i32,
            comment_count: i32,
            tags: Vec<()> // TODO: Vec<Tag>
        ) -> Self {
        Self {
            id: model.id,
            thumbnail_url,
            r#type: model.r#type.clone(),
            safety: model.safety.clone(),
            score,
            favorite_count,
            comment_count,
            tags,
            version: model.version,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PostAnswer {
    pub id: i32,
    pub version: i32,
    #[serde(rename = "creationTime")]
    pub creation_time: NaiveDateTime,
    #[serde(rename = "lastEditTime")]
    pub last_edit_time: Option<NaiveDateTime>,
    pub safety: String,
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub checksum: String,
    #[serde(rename = "checksumMD5")]
    pub checksum_md5: Option<String>,
    #[serde(rename = "fileSize")]
    pub file_size: Option<i64>,
    #[serde(rename = "canvasWidth")]
    pub canvas_width: Option<i32>,
    #[serde(rename = "canvasHeight")]
    pub canvas_height: Option<i32>,
    #[serde(rename = "contentUrl")]
    pub content_url: String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,
    pub flags: Vec<String>,
    pub tags: Vec<()>, // Vec<Tag>
    pub relations: Vec<()>,
    pub user: Option<User>,
    pub score: i64,
    #[serde(rename = "ownScore")]
    pub own_score: i64,
    #[serde(rename = "ownFavorite")]
    pub own_favorite: bool,
    #[serde(rename = "tagCount")]
    pub tag_count: i64,
    #[serde(rename = "favoriteCount")]
    pub favorite_count: i64,
    #[serde(rename = "commentCount")]
    pub comment_count: i64,
    #[serde(rename = "noteCount")]
    pub note_count: i64,
    #[serde(rename = "relationCount")]
    pub relation_count: i64,
    #[serde(rename = "featureCount")]
    pub feature_count: i64,
    #[serde(rename = "lastFeatureTime")]
    pub last_feature_time: Option<()>,
    #[serde(rename = "favoritedBy")]
    pub favorited_by: Vec<()>,
    #[serde(rename = "hasCustomThumbnail")]
    pub has_custom_thumbnail: bool,
    pub notes: Vec<()>,
    pub comments: Vec<()>,
    pub pools: Vec<()>,
}


// #[derive(Serialize, Deserialize)]
// pub struct Tag {
//     pub names: Vec<String>,
//     pub category: String,
//     pub usages: i64,
// }

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
}

#[derive(Debug, Deserialize)]
pub struct PostsParams {
    pub query: String,
    pub offset: Option<u64>,
    pub limit: u64,
    pub fields: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReverseSearchQuery {
    pub content_token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReverseSearchAnswer {
    pub exact_post: Option<String>,
    pub similar_posts: Vec<()>,
}