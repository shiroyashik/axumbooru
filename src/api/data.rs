use axum::{extract::{Multipart, State}, routing::get_service, Json, Router};
use serde::Serialize;
use tower_http::services::ServeDir;
use log::{debug, info};
use std::{fs, sync::Arc};

use crate::{data::DATA, error::{ApiError, ApiResult}, AppState};

pub fn data_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(DATA)))
}

pub fn get_folder_size(path: &str) -> Result<u64, std::io::Error> {
    // Получение списка элементов в папке
    let entries = fs::read_dir(path)?;
    // Инициализация переменной для хранения размера папки
    let mut total_size: u64 = 0;
    // Итерация по элементам папки
    for entry in entries {
        // Получение метаданных элемента
        let metadata = entry?.metadata()?;
        // Если элемент является файлом, добавляем его размер к общему размеру
        if metadata.is_file() {
            total_size += metadata.len();
        }
    }
    debug!("Data dir size: {total_size}");
    // Возвращение общего размера папки
    Ok(total_size)
}

#[derive(Serialize)]
pub struct UploadResponse {
    pub token: String,
}

fn to_upload_error(_: anyhow::Error) -> ApiError {
    ApiError::Uploads
}

pub async fn upload(State(state): State<Arc<AppState>>, mut multipart: Multipart) -> ApiResult<Json<UploadResponse>> {
    let mut token: Option<String> = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        // debug!("Multipart: {:?} {:?} {:?} {:?}", field.content_type(), field.file_name(), field.name(), field.headers());
        if name == "content".to_string() {
            let filename = field.file_name().unwrap().to_owned();
            let content_type = field.content_type().unwrap().to_owned();
            let (_, extension) = filename.split_once('.').expect("Damaged file");
            let data = field.bytes().await.unwrap();
            token = Some(state.uploads.lock().expect("Uploads mutex was poisoned!").add(extension, &content_type, data).map_err(to_upload_error)?);
        }
    }
    match token {
        Some(token) => {
            info!("Responding token: {token}");
            return Ok(Json(UploadResponse { token }))
        },
        None => {
            panic!("DOESN'T HAVE CONTENT!")
        },
    }
}