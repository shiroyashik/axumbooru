/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use crate::models::users::User;

type Connection = diesel::pg::PgConnection;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable, Associations, Selectable)]
#[diesel(table_name=posts, primary_key(id), belongs_to(User, foreign_key=user_id))]
pub struct Post {
    pub id: i32,
    pub user_id: Option<i32>,
    pub creation_time: chrono::NaiveDateTime,
    pub last_edit_time: Option<chrono::NaiveDateTime>,
    pub safety: String,
    pub type_: String,
    pub checksum: String,
    pub source: Option<String>,
    pub file_size: Option<i64>,
    pub image_width: Option<i32>,
    pub image_height: Option<i32>,
    pub mime_type: String,
    pub version: i32,
    pub flags: Option<String>,
    pub checksum_md5: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=posts)]
pub struct CreatePost {
    pub id: i32,
    pub user_id: Option<i32>,
    pub creation_time: chrono::NaiveDateTime,
    pub last_edit_time: Option<chrono::NaiveDateTime>,
    pub safety: String,
    pub type_: String,
    pub checksum: String,
    pub source: Option<String>,
    pub file_size: Option<i64>,
    pub image_width: Option<i32>,
    pub image_height: Option<i32>,
    pub mime_type: String,
    pub version: i32,
    pub flags: Option<String>,
    pub checksum_md5: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=posts)]
pub struct UpdatePost {
    pub user_id: Option<Option<i32>>,
    pub creation_time: Option<chrono::NaiveDateTime>,
    pub last_edit_time: Option<Option<chrono::NaiveDateTime>>,
    pub safety: Option<String>,
    pub type_: Option<String>,
    pub checksum: Option<String>,
    pub source: Option<Option<String>>,
    pub file_size: Option<Option<i64>>,
    pub image_width: Option<Option<i32>>,
    pub image_height: Option<Option<i32>>,
    pub mime_type: Option<String>,
    pub version: Option<i32>,
    pub flags: Option<Option<String>>,
    pub checksum_md5: Option<Option<String>>,
}


#[derive(Debug, Serialize)]
pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    /// 0-based index
    pub page: i64,
    pub page_size: i64,
    pub num_pages: i64,
}

impl Post {

    pub fn create(db: &mut Connection, item: &CreatePost) -> QueryResult<Self> {
        use crate::schema::posts::dsl::*;

        insert_into(posts).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut Connection, param_id: i32) -> QueryResult<Self> {
        use crate::schema::posts::dsl::*;

        posts.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::posts::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = posts.count().get_result(db)?;
        let items = posts.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn update(db: &mut Connection, param_id: i32, item: &UpdatePost) -> QueryResult<Self> {
        use crate::schema::posts::dsl::*;

        diesel::update(posts.filter(id.eq(param_id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::posts::dsl::*;

        diesel::delete(posts.filter(id.eq(param_id))).execute(db)
    }

}