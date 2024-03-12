use ::entity::{post, post::Entity as Post};
use chrono::Local;
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_posts_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<post::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Post::find()
            .order_by_asc(post::Column::Id)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginator posts
        paginator.fetch_page(page).await.map(|p| (p, num_pages))
    }

    pub async fn count_posts(db: &DbConn) -> Result<u64, DbErr> {
        Post::find().count(db).await
    }

    pub async fn find_post_by_id(db: &DbConn, id: i32) -> Result<Option<post::Model>, DbErr> {
        Post::find_by_id(id).one(db).await
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn create_post(
        db: &DbConn,
        form_data: post::Model,
    ) -> Result<post::ActiveModel, DbErr> {
        post::ActiveModel {
            creation_time: Set(Local::now().naive_local().to_owned()),
            safety: Set(form_data.safety.to_owned()),
            r#type: Set(form_data.r#type.to_owned()),
            checksum: Set(form_data.checksum.to_owned()),
            mime_type: Set(form_data.mime_type.to_owned()),
            version: Set(form_data.version.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_post_by_id(
        db: &DbConn,
        id: i32,
        form_data: post::Model,
    ) -> Result<post::Model, DbErr> {
        let posts: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;
        post::ActiveModel {
            id: posts.id,
            user_id: posts.user_id,
            creation_time: posts.creation_time,
            last_edit_time: Set(Some(Local::now().naive_local().to_owned())),
            safety: Set(form_data.safety.to_owned()),
            r#type: Set(form_data.r#type.to_owned()),
            checksum: Set(form_data.checksum.to_owned()),
            source: posts.source,
            file_size: posts.file_size,
            image_width: posts.image_width,
            image_height: posts.image_height,
            mime_type: Set(form_data.mime_type.to_owned()),
            version: Set(form_data.version.to_owned()),
            flags: posts.flags,
            checksum_md5: posts.checksum_md5,
        }
        .update(db)
        .await
    }

    pub async fn delete_post(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    // pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
    //     Post::delete_many().exec(db).await
    // }
}