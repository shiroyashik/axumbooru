use std::{fmt::Display, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use log::debug;
use md5::Md5;
use hmac::{Hmac, Mac};

type HmacMd5 = Hmac<Md5>;

use crate::{
    error::ApiResult, AppState,
};

#[derive(Serialize, Deserialize)]
pub struct PostsAnswer {
    pub query: String,
    pub offset: u64,
    pub limit: u64,
    pub total: u64,
    pub results: Vec<Results>,
}

#[derive(Serialize, Deserialize)]
pub struct Results {
    pub id: i32,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub safety: String,
    pub score: i64,
    #[serde(rename = "favoriteCount")]
    pub favorite_count: i64,
    #[serde(rename = "commentCount")]
    pub comment_count: i64,
    pub tags: Vec<()>, // Vec<Tag>
    pub version: i32,
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
pub struct PostsAnswerQuery {
    pub query: String,
    pub offset: Option<u64>,
    pub limit: u64,
    pub fields: String,
}

pub async fn list_of_posts(
    Query(params): Query<PostsAnswerQuery>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<PostsAnswer>> {
    debug!("{params:?}");
    // let fields_mas = get_fields_from_string(params.fields.clone());
    // debug!("{:?}", &fields_mas);
    // oki

    let total = state.db.get_posts_count().await?;
    let offset = {
        match params.offset {
            None => 0,
            Some(i) => i,
        }
    };

    let (results_raw, _) = state.db.get_posts_in_page(offset, params.limit).await?;
        // PostQuery::find_posts_in_page_with_filter(&state.db, offset, fields_mas, params.limit).await.unwrap();
    debug!("{results_raw:?}");
    let mut results: Vec<Results> = Vec::new();
    for model in results_raw.iter() {
        results.push(Results {
            id: model.id,
            thumbnail_url: "/data/2.jpg".to_string(),
            type_field: model.r#type.clone(),
            safety: model.safety.clone(),
            score: 0,
            favorite_count: 0,
            comment_count: 0,
            tags: Vec::new(),
            version: model.version,
        })
    }

    let posts = PostsAnswer {
        query: params.fields,
        offset,
        limit: params.limit,
        total,
        results,
    };

    Ok(Json(posts))
    // end
}

pub async fn get_post_by_id(
    Path(id): Path<u64>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<PostAnswer>> {
    let raw_post = state.db.get_post_by_id(id).await?;
    
    let mut flags: Vec<String> = Vec::new();
    if raw_post.flags.is_some() {
        for part in raw_post.flags.unwrap().split(",") {
            flags.push(part.to_string());
        }
    }

    let post = PostAnswer {
        id: raw_post.id,
        version: raw_post.version,
        creation_time: raw_post.creation_time,
        last_edit_time: raw_post.last_edit_time,
        safety: raw_post.safety,
        source: raw_post.source,
        type_field: raw_post.r#type,
        mime_type: raw_post.mime_type.clone(),
        checksum: raw_post.checksum,
        checksum_md5: raw_post.checksum_md5,
        file_size: raw_post.file_size,
        canvas_width: raw_post.image_width,
        canvas_height: raw_post.image_height,
        content_url: get_post_content_path(raw_post.id, get_post_security_hash(id, &state.config.secret), &raw_post.mime_type),
        thumbnail_url: get_post_thumbnail_path(raw_post.id, get_post_security_hash(id, &state.config.secret)),
        flags,
        tags: Vec::new(),
        relations: Vec::new(),
        user: None, // !!!
        score: 0,
        own_score: 0,
        own_favorite: false,
        tag_count: 0,
        favorite_count: 0,
        comment_count: 0,
        note_count: 0,
        relation_count: 0,
        feature_count: 0,
        last_feature_time: None,
        favorited_by: Vec::new(),
        has_custom_thumbnail: false,
        notes: Vec::new(),
        comments: Vec::new(),
        pools: Vec::new(),
    };

    Ok(Json(post))

    // return Err(crate::ErrorStruct::new(
    //     "TODO".to_string(),
    //     "TODO".to_string(),
    // ));
}

pub fn get_post_security_hash<T: ToString>(id: T, key: &str) -> String {
    use std::fmt::Write;
    let mut mac = HmacMd5::new_from_slice(key.as_bytes()).expect("Something wrong with HMAC key!");
    mac.update(id.to_string().as_bytes());
    let code = &mac.finalize().into_bytes()[0 .. 8]; // wtf how this work but im need 16 chars, and 8 getting 16 chars
    let mut result = String::new();
    for byte in code {
      write!(result, "{:02x}", byte).unwrap();
    }
    result
}

pub fn get_post_content_path<T: Display>(id: T, hash: String, mime: &str) -> String {
    let extension = mime_guess2::get_mime_extensions_str(mime).expect("Unknown mime type!")[0];
    format!("data/posts/{id}_{hash}.{extension}").to_string()
}

pub fn get_post_thumbnail_path<T: Display>(id: T, hash: String) -> String {
    format!("data/generated-thumbnails/{id}_{hash}.jpg").to_string()
}