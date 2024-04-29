use std::{fmt::Write as _, fs, path::Path};
use anyhow::{anyhow, Ok, Result};
use axum::body::Bytes;
use dashmap::DashMap;
use log::{debug, info, warn};
use ring::digest;

#[derive(Debug)]
pub struct Data(DashMap<String, Upload>);

pub const DATA: &str = "./data";
pub const AVATARS: &str = "./data/avatars";
pub const POSTS: &str = "./data/posts";
pub const TEMP: &str = "./data/temporary-uploads";
pub const THUMBNAILS: &str = "./data/generated-thumbnails";

#[derive(Debug, Clone)]
pub struct Upload {
    pub filename: String,
    pub content_type: String,
}

impl Data {
    pub fn new() -> Self {
        Self::default()
    }
    // Working with data dir
    fn check_and_repair_directory(path: &str) -> Result<()> {
        if !Path::new(path).is_dir() {
            warn!("{} not found", path);
            fs::create_dir(path)?;
            info!("{} created!", path);
        }
        Ok(())
    }
    pub fn repair_data() -> Result<()> {
        debug!("Data Storage repair started!");
        Data::check_and_repair_directory(DATA)?;
        Data::check_and_repair_directory(AVATARS)?;
        Data::check_and_repair_directory(POSTS)?;
        Data::check_and_repair_directory(TEMP)?;
        Data::check_and_repair_directory(THUMBNAILS)?;
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
    // Implementing Self
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