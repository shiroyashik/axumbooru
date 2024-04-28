use std::{fmt::Write as _, fs};
use anyhow::{anyhow, Ok, Result};
use axum::body::Bytes;
use dashmap::DashMap;
use log::debug;
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
    pub fn flush_temporary_uploads() -> Result<()> {
        debug!("Flush started!");
        for file in fs::read_dir("./data/tmp")? {
            // debug!("Removing {:?}", file?.file_name());
            fs::remove_file(file?.path())?;
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
        fs::write(format!("./data/tmp/{filename}"), raw)?;
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