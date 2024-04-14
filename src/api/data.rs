use axum::{body::Bytes, extract::{Multipart, State}, routing::get_service, Json, Router};
use serde::Serialize;
use sha1::{Digest, Sha1};
use tower_http::services::ServeDir;
use log::{debug, info};
use std::{fmt::Write as _, fs, sync::Arc};

use crate::{AppState, Result as HomebrewResult};

pub fn data_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./data")))
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

#[derive(Debug)]
pub struct Uploads {
    file_name: String,
    content_type: String,
    size: usize,
}

pub async fn upload(State(state): State<Arc<AppState>>, mut multipart: Multipart) -> HomebrewResult<Json<UploadResponse>> {
    let mut data = Bytes::new();
    let mut upload_meta: Option<Uploads> = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        // debug!("Multipart: {:?} {:?} {:?} {:?}", field.content_type(), field.file_name(), field.name(), field.headers());
        if name == "content".to_string() {
            debug!("Trying to catch file-name...");
            let file_name = field.file_name().unwrap().to_owned();
            debug!("Trying to catch content-type...");
            let content_type = field.content_type().unwrap().to_owned();
            data = field.bytes().await.unwrap();
            fs::write(format!("./data/tmp/{file_name}"), data.clone()).unwrap();
            upload_meta = Some(Uploads { file_name, content_type, size: data.len() });
        }
    } 
    let mut token = String::new();
    match upload_meta {
        Some(_) => {
            let mut hasher = Sha1::new();
            hasher.update(data.as_ref());
            let raw_token = &*hasher.finalize();
            // Converting [u8] array to String
            for byte in raw_token {
              write!(token, "{:02x}", byte).unwrap();
            }
        },
        None => {
            panic!("DOESN'T HAVE CONTENT!")
        },
    }
    debug!("Storing upload metadata into state...");
    let mut state = state.uploads.lock().expect("Uploads mutex was poisoned!");
    state.insert(token.clone(), upload_meta.unwrap());
    info!("Responding token: {token}");
    Ok(Json(UploadResponse { token }))
}