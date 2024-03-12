use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;
use tracing::debug;
use std::fs;

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