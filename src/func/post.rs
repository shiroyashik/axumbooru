use std::fmt::Display;
use md5::Md5;
use hmac::{Hmac, Mac};

type HmacMd5 = Hmac<Md5>;

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