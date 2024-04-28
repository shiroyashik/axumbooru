use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use log::debug;

use crate::{
    error::{ApiError, ApiResult}, AppState, func::post::*
};
use super::model::*;

pub async fn list_of_posts(
    Query(params): Query<PostsParams>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ListOfPostsAnswer>> {
    debug!("Post listing params: {params:?}");
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
    let mut results: Vec<MiniPost> = Vec::new();
    for model in results_raw.iter() {
        results.push(MiniPost::from_model(model, "data/avatarka.jpg".to_string(), 0, 0, 0, Vec::new()))
    }   // TODO: заглушки :(

    let posts = ListOfPostsAnswer {
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
        flags, // TODO: Дальше чисто заглушки
        tags: Vec::new(),
        relations: Vec::new(),
        user: None, 
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
}

pub async fn reverse_post_search(
    State(state): State<Arc<AppState>>,
    Json(content_path):  Json<ReverseSearchQuery>
) -> ApiResult<Json<ReverseSearchAnswer>> {
    // TODO: Пока что здесь просто заглушка
    if !state.uploads.lock().expect("Uploads mutex was poisoned!").is_existing(&content_path.content_token) {
        return Err(ApiError::Uploads);
    };
    Ok(Json(ReverseSearchAnswer {
        exact_post: None,
        similar_posts: Vec::new(),
    }))
}