use std::{io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::UserRank;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub name: String,
    pub domain: String,
    pub listen: String,
    pub secret: String,
    pub delete_source_files: bool,
    pub contact_email: String,
    pub enable_safety: bool,
    pub tag_name_regex: String,
    pub tag_category_name_regex: String,
    pub pool_name_regex: String,
    pub pool_category_name_regex: String,
    pub password_regex: String,
    pub user_name_regex: String,
    pub allow_broken_uploads: bool,
    pub webhooks: Option<Vec<String>>,
    pub default_rank: UserRank,
    pub thumbnails: Thumbnails,
    pub smtp: Smtp,
    pub privileges: Privileges,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Thumbnails {
    pub avatar_width: u64,
    pub avatar_height: u64,
    pub post_width: u64,
    pub post_height: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Smtp {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pass: String,
    pub from: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Privileges {
    #[serde(rename = "users:create:self")]
    pub users_create_self: UserRank,
    #[serde(rename = "users:create:any")]
    pub users_create_any: UserRank,
    #[serde(rename = "users:list")]
    pub users_list: UserRank,
    #[serde(rename = "users:view")]
    pub users_view: UserRank,
    #[serde(rename = "users:edit:any:name")]
    pub users_edit_any_name: UserRank,
    #[serde(rename = "users:edit:any:pass")]
    pub users_edit_any_pass: UserRank,
    #[serde(rename = "users:edit:any:email")]
    pub users_edit_any_email: UserRank,
    #[serde(rename = "users:edit:any:avatar")]
    pub users_edit_any_avatar: UserRank,
    #[serde(rename = "users:edit:any:rank")]
    pub users_edit_any_rank: UserRank,
    #[serde(rename = "users:edit:self:name")]
    pub users_edit_self_name: UserRank,
    #[serde(rename = "users:edit:self:pass")]
    pub users_edit_self_pass: UserRank,
    #[serde(rename = "users:edit:self:email")]
    pub users_edit_self_email: UserRank,
    #[serde(rename = "users:edit:self:avatar")]
    pub users_edit_self_avatar: UserRank,
    #[serde(rename = "users:edit:self:rank")]
    pub users_edit_self_rank: UserRank,
    #[serde(rename = "users:delete:any")]
    pub users_delete_any: UserRank,
    #[serde(rename = "users:delete:self")]
    pub users_delete_self: UserRank,
    #[serde(rename = "userTokens:list:any")]
    pub user_tokens_list_any: UserRank,
    #[serde(rename = "userTokens:list:self")]
    pub user_tokens_list_self: UserRank,
    #[serde(rename = "userTokens:create:any")]
    pub user_tokens_create_any: UserRank,
    #[serde(rename = "userTokens:create:self")]
    pub user_tokens_create_self: UserRank,
    #[serde(rename = "userTokens:edit:any")]
    pub user_tokens_edit_any: UserRank,
    #[serde(rename = "userTokens:edit:self")]
    pub user_tokens_edit_self: UserRank,
    #[serde(rename = "userTokens:delete:any")]
    pub user_tokens_delete_any: UserRank,
    #[serde(rename = "userTokens:delete:self")]
    pub user_tokens_delete_self: UserRank,
    #[serde(rename = "posts:create:anonymous")]
    pub posts_create_anonymous: UserRank,
    #[serde(rename = "posts:create:identified")]
    pub posts_create_identified: UserRank,
    #[serde(rename = "posts:list")]
    pub posts_list: UserRank,
    #[serde(rename = "posts:reverseSearch")]
    pub posts_reverse_search: UserRank,
    #[serde(rename = "posts:view")]
    pub posts_view: UserRank,
    #[serde(rename = "posts:view:featured")]
    pub posts_view_featured: UserRank,
    #[serde(rename = "posts:edit:content")]
    pub posts_edit_content: UserRank,
    #[serde(rename = "posts:edit:flags")]
    pub posts_edit_flags: UserRank,
    #[serde(rename = "posts:edit:notes")]
    pub posts_edit_notes: UserRank,
    #[serde(rename = "posts:edit:relations")]
    pub posts_edit_relations: UserRank,
    #[serde(rename = "posts:edit:safety")]
    pub posts_edit_safety: UserRank,
    #[serde(rename = "posts:edit:source")]
    pub posts_edit_source: UserRank,
    #[serde(rename = "posts:edit:tags")]
    pub posts_edit_tags: UserRank,
    #[serde(rename = "posts:edit:thumbnail")]
    pub posts_edit_thumbnail: UserRank,
    #[serde(rename = "posts:feature")]
    pub posts_feature: UserRank,
    #[serde(rename = "posts:delete")]
    pub posts_delete: UserRank,
    #[serde(rename = "posts:score")]
    pub posts_score: UserRank,
    #[serde(rename = "posts:merge")]
    pub posts_merge: UserRank,
    #[serde(rename = "posts:favorite")]
    pub posts_favorite: UserRank,
    #[serde(rename = "posts:bulk-edit:tags")]
    pub posts_bulk_edit_tags: UserRank,
    #[serde(rename = "posts:bulk-edit:safety")]
    pub posts_bulk_edit_safety: UserRank,
    #[serde(rename = "posts:bulk-edit:delete")]
    pub posts_bulk_edit_delete: UserRank,
    #[serde(rename = "tags:create")]
    pub tags_create: UserRank,
    #[serde(rename = "tags:edit:names")]
    pub tags_edit_names: UserRank,
    #[serde(rename = "tags:edit:category")]
    pub tags_edit_category: UserRank,
    #[serde(rename = "tags:edit:description")]
    pub tags_edit_description: UserRank,
    #[serde(rename = "tags:edit:implications")]
    pub tags_edit_implications: UserRank,
    #[serde(rename = "tags:edit:suggestions")]
    pub tags_edit_suggestions: UserRank,
    #[serde(rename = "tags:list")]
    pub tags_list: UserRank,
    #[serde(rename = "tags:view")]
    pub tags_view: UserRank,
    #[serde(rename = "tags:merge")]
    pub tags_merge: UserRank,
    #[serde(rename = "tags:delete")]
    pub tags_delete: UserRank,
    #[serde(rename = "tagCategories:create")]
    pub tag_categories_create: UserRank,
    #[serde(rename = "tagCategories:edit:name")]
    pub tag_categories_edit_name: UserRank,
    #[serde(rename = "tagCategories:edit:color")]
    pub tag_categories_edit_color: UserRank,
    #[serde(rename = "tagCategories:edit:order")]
    pub tag_categories_edit_order: UserRank,
    #[serde(rename = "tagCategories:list")]
    pub tag_categories_list: UserRank,
    #[serde(rename = "tagCategories:view")]
    pub tag_categories_view: UserRank,
    #[serde(rename = "tagCategories:delete")]
    pub tag_categories_delete: UserRank,
    #[serde(rename = "tagCategories:setDefault")]
    pub tag_categories_set_default: UserRank,
    #[serde(rename = "pools:create")]
    pub pools_create: UserRank,
    #[serde(rename = "pools:edit:names")]
    pub pools_edit_names: UserRank,
    #[serde(rename = "pools:edit:category")]
    pub pools_edit_category: UserRank,
    #[serde(rename = "pools:edit:description")]
    pub pools_edit_description: UserRank,
    #[serde(rename = "pools:edit:posts")]
    pub pools_edit_posts: UserRank,
    #[serde(rename = "pools:list")]
    pub pools_list: UserRank,
    #[serde(rename = "pools:view")]
    pub pools_view: UserRank,
    #[serde(rename = "pools:merge")]
    pub pools_merge: UserRank,
    #[serde(rename = "pools:delete")]
    pub pools_delete: UserRank,
    #[serde(rename = "poolCategories:create")]
    pub pool_categories_create: UserRank,
    #[serde(rename = "poolCategories:edit:name")]
    pub pool_categories_edit_name: UserRank,
    #[serde(rename = "poolCategories:edit:color")]
    pub pool_categories_edit_color: UserRank,
    #[serde(rename = "poolCategories:list")]
    pub pool_categories_list: UserRank,
    #[serde(rename = "poolCategories:view")]
    pub pool_categories_view: UserRank,
    #[serde(rename = "poolCategories:delete")]
    pub pool_categories_delete: UserRank,
    #[serde(rename = "poolCategories:setDefault")]
    pub pool_categories_set_default: UserRank,
    #[serde(rename = "comments:create")]
    pub comments_create: UserRank,
    #[serde(rename = "comments:delete:any")]
    pub comments_delete_any: UserRank,
    #[serde(rename = "comments:delete:own")]
    pub comments_delete_own: UserRank,
    #[serde(rename = "comments:edit:any")]
    pub comments_edit_any: UserRank,
    #[serde(rename = "comments:edit:own")]
    pub comments_edit_own: UserRank,
    #[serde(rename = "comments:list")]
    pub comments_list: UserRank,
    #[serde(rename = "comments:view")]
    pub comments_view: UserRank,
    #[serde(rename = "comments:score")]
    pub comments_score: UserRank,
    #[serde(rename = "snapshots:list")]
    pub snapshots_list: UserRank,
    #[serde(rename = "uploads:create")]
    pub uploads_create: UserRank,
    #[serde(rename = "uploads:useDownloader")]
    pub uploads_use_downloader: UserRank,
}

impl Config {
    pub fn parse(path: PathBuf) -> Self {
        let mut file = std::fs::File::open(path).expect("Access denied or file doesn't exists!");
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        toml::from_str(&data).unwrap()
    }
}

// #[allow(dead_code)]
// impl Default for Config {
//     fn default() -> Self {
//         Self {
//             name: "axumbooru".to_string(),
//         }
//     }
// }
