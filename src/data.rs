use std::{fmt::Write as _, fs, path::Path};
use anyhow::{anyhow, Ok, Result};
use axum::body::Bytes;
use dashmap::DashMap;
use log::{debug, info, warn};
use ring::digest;

#[derive(Debug)]
pub struct Data(DashMap<String, Upload>);

#[derive(Debug, Clone)]
pub struct Upload {
    pub filename: String,
    pub content_type: String,
}

impl Data {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn repair_data() -> Result<()> {
        debug!("Data Storage repair started!");
        if !Path::new("./data").is_dir() {
            warn!("./data not found");
            fs::create_dir("./data")?;
            info!("./data created!");
        }
        if !Path::new("./data/temporary-uploads").is_dir() {
            warn!("./data/temporary-uploads not found");
            fs::create_dir("./data/temporary-uploads")?;
            info!("./data/temporary-uploads created!");
        }
        if !Path::new("./data/avatars").is_dir() {
            warn!("./data/avatars not found");
            fs::create_dir("./data/avatars")?;
            info!("./data/avatars created!");
        }
        if !Path::new("./data/posts").is_dir() {
            warn!("./data/posts not found");
            fs::create_dir("./data/posts")?;
            info!("./data/posts created!");
        }
        if !Path::new("./data/generated-thumbnails").is_dir() {
            warn!("./data/generated-thumbnails not found");
            fs::create_dir("./data/generated-thumbnails")?;
            info!("./data/generated-thumbnails created!");
        }
        debug!("Data Storage repair complete!");
        Ok(())
    }
    pub fn flush_temporary_uploads() -> Result<()> {
        debug!("Flush started!");
        for file in fs::read_dir("./data/temporary-uploads")? {
            let file = file?;
            debug!("Removing {:?}", &file.file_name());
            fs::remove_file(file.path())?;
        }
        debug!("Flushing complete!");
        Ok(())
    }
    pub fn vec(&self) -> Vec<(String, Upload)> {
        self.0.clone().into_iter().collect()
    }
    pub fn get_and_remove(&self, token: &str) -> Option<Upload> {
        Some(self.0.remove(token)?.to_owned().1)
    }
    pub fn is_existing(&self, token: &str) -> bool {
        self.0.contains_key(token)
    }
    pub fn add(&self, extension: &str, content_type: &str, raw: Bytes) -> Result<String> {
        let binding = digest::digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, &raw);
        let hash = binding.as_ref();
        // let hash = String::from_utf8(hash.to_vec())?;
        let mut token = String::new();
        for byte in hash {
            write!(token, "{:02x}", byte).unwrap();
        }
        if self.is_existing(&token) {
            return Err(anyhow!("{token} is existing!"));
        };
        let filename = format!("{}.{}", token, extension);
        fs::write(format!("./data/temporary-uploads/{filename}"), raw)?;
        self.0.insert(token.clone(), Upload { filename, content_type: content_type.to_owned() });
        Ok(token)
    }
}

impl Default for Data {
    fn default() -> Self {
        Self(DashMap::new())
    }
}

/*

Работа с временными файлами
 - Очистить директорию
 - Получить список в виде Vec
 - Проверить существование
 - Добавить новый файл в список и вернуть контент токен
 - Получить по токену структуру Uploads

*/