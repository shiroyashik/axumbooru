use sea_orm::*;
use chrono::Local;

use crate::db::schemas::{
    prelude::*,
    user, user_token, post, snapshot,
};
use super::errors::*;

pub fn to_db_error(e: sea_orm::DbErr) -> DatabaseError {
    DatabaseError::from(anyhow::Error::from(e))
}

#[derive(Debug, Clone)]
pub struct Repository(DatabaseConnection);

impl Repository {
    // Creating Structure
    pub async fn create(url: String) -> anyhow::Result<Repository> {
        let pool = Database::connect(url).await?;
        Ok(Repository(pool))
    }
    pub fn with_connection(pool: DatabaseConnection) -> Repository {
        Repository(pool)
    }

    pub fn pool(&self) -> DatabaseConnection {
        self.0.clone()
    }
    // User
    pub async fn get_users_count(&self) -> Result<u64, DatabaseError> {
        User::find().count(&self.0).await.map_err(to_db_error)
    }
    pub async fn get_users_in_page(&self, page: u64, per_page: u64) -> Result<(Vec<user::Model>, u64), DatabaseError> {
        // Setup paginator
        let paginator = User::find()
            .order_by_asc(user::Column::Id)
            .paginate(&self.0, per_page);
        let num_pages = paginator.num_pages().await.map_err(to_db_error)?;
        // Fetch paginator users
        paginator.fetch_page(page - 1).await.map_err(to_db_error).map(|p| (p, num_pages))
    }
    pub async fn get_user_by_id(&self, id: u64) -> Result<user::Model, GetUserError> {
        Ok(User::find_by_id(id as i32).one(&self.0).await.map_err(to_db_error)?.ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("User not found"))})?)
    }
    pub async fn get_user_by_name(&self, name: &str) -> Result<user::Model, GetUserError> {
        let user = User::find()
            .filter(user::Column::Name.contains(name))
            .one(&self.0)
            .await.map_err(to_db_error)?
            .ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("User not found"))})?;
        Ok(user)
    }
    pub async fn create_user(&self, user: user::ActiveModel) -> Result<user::ActiveModel, DatabaseError> {
        let user = user.try_into_model().expect("Can't into model");
        user::ActiveModel {
            name: Set(user.name.to_owned()),
            password_hash: Set(user.password_hash.to_owned()),
            password_salt: Set(user.password_salt.to_owned()),
            email: Set(user.email.to_owned()),
            rank: Set(user.rank.to_owned()),
            creation_time: Set(Local::now().naive_local().to_owned()),
            avatar_style: Set(user.avatar_style.to_owned()),
            version: Set(1),
            password_revision: Set(3),
            ..Default::default()
        }
        .save(&self.0)
        .await.map_err(to_db_error)
    }
    pub async fn update_user(&self, id: u64, user: user::ActiveModel) -> Result<user::Model, DatabaseError> {
        let current_user: user::ActiveModel = User::find_by_id(id as i32)
            .one(&self.0)
            .await.map_err(to_db_error)?
            .ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("User not found"))})
            .map(Into::into)?;
        let user = user.try_into_model().expect("Can't into model");
        user::ActiveModel {
            id: current_user.id,
            name: current_user.name,
            password_hash: Set(user.password_hash.to_owned()),
            password_salt: Set(user.password_salt.to_owned()),
            email: Set(user.email.to_owned()),
            rank: Set(user.rank.to_owned()),
            creation_time: current_user.creation_time,
            last_login_time: current_user.last_login_time,
            avatar_style: Set(user.avatar_style.to_owned()),
            version: Set(user.version.to_owned()),
            password_revision: Set(user.password_revision.to_owned()),
        }
        .update(&self.0)
        .await.map_err(to_db_error)
    }
    pub async fn delete_user(&self, id: u64) -> Result<(), DatabaseError> {
        let user: user::ActiveModel = User::find_by_id(id as i32)
            .one(&self.0)
            .await.map_err(to_db_error)?
            .ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("User not found"))})
            .map(Into::into)?;

        user.delete(&self.0).await.map_err(to_db_error)?;
        Ok(())
    }
    pub async fn update_last_login_time(&self, name: &str) -> Result<user::Model, DatabaseError> {
        let mut current_user: user::ActiveModel = User::find()
            .filter(user::Column::Name.contains(name))
            .one(&self.0)
            .await.map_err(to_db_error)?
            .ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("User not found"))})
            .map(Into::into)?;
        current_user.last_login_time = Set(Some(Local::now().naive_local().to_owned()));
        current_user.update(&self.0).await.map_err(to_db_error)
    }
    // Post
    pub async fn get_posts_count(&self) -> Result<u64, DatabaseError> {
        Post::find().count(&self.0).await.map_err(to_db_error)
    }
    pub async fn get_posts_in_page(&self, page: u64, per_page: u64) -> Result<(Vec<post::Model>, u64), DatabaseError> {
        // Setup paginator
        let paginator = Post::find()
            .order_by_asc(post::Column::Id)
            .paginate(&self.0, per_page);
        let num_pages = paginator.num_pages().await.map_err(to_db_error)?;
        // Fetch paginator posts
        paginator.fetch_page(page).await.map_err(to_db_error).map(|p| (p, num_pages))
    }
    pub async fn get_post_by_id(&self, id: u64) -> Result<post::Model, DatabaseError> {
        Ok(Post::find_by_id(id as i32).one(&self.0).await.map_err(to_db_error)?.ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("Post not found"))})?)
    }
    pub async fn create_post(&self, post: post::ActiveModel) -> Result<post::ActiveModel, DatabaseError> {
        let post = post.try_into_model().expect("Can't into model");
        post::ActiveModel {
            creation_time: Set(Local::now().naive_local().to_owned()),
            safety: Set(post.safety.to_owned()),
            r#type: Set(post.r#type.to_owned()),
            checksum: Set(post.checksum.to_owned()),
            mime_type: Set(post.mime_type.to_owned()),
            version: Set(post.version.to_owned()),
            ..Default::default()
        }
        .save(&self.0)
        .await.map_err(to_db_error)
    }
    pub async fn update_post(&self, id: u64, post: post::ActiveModel) -> Result<post::Model, DatabaseError> {
        let posts: post::ActiveModel = Post::find_by_id(id as i32)
            .one(&self.0)
            .await.map_err(to_db_error)?
            .ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("Post not found"))})
            .map(Into::into)?;
        let post = post.try_into_model().expect("Can't into model");
        post::ActiveModel {
            id: posts.id,
            user_id: posts.user_id,
            creation_time: posts.creation_time,
            last_edit_time: Set(Some(Local::now().naive_local().to_owned())),
            safety: Set(post.safety.to_owned()),
            r#type: Set(post.r#type.to_owned()),
            checksum: Set(post.checksum.to_owned()),
            source: posts.source,
            file_size: posts.file_size,
            image_width: posts.image_width,
            image_height: posts.image_height,
            mime_type: Set(post.mime_type.to_owned()),
            version: Set(post.version.to_owned()),
            flags: posts.flags,
            checksum_md5: posts.checksum_md5,
        }
        .update(&self.0)
        .await.map_err(to_db_error)
    }
    pub async fn delete_post(&self, id: u64) -> Result<(), DatabaseError> {
        let post: post::ActiveModel = Post::find_by_id(id as i32)
            .one(&self.0)
            .await.map_err(to_db_error)?
            .ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("Post not found"))})
            .map(Into::into)?;
        post.delete(&self.0).await.map_err(to_db_error)?;
        Ok(())
    }
    // User Token
    pub async fn get_user_tokens_count(&self) -> Result<u64, DatabaseError> {
        UserToken::find().count(&self.0).await.map_err(to_db_error)
    }
    pub async fn get_user_tokens_in_page(&self, page: u64, per_page: u64) -> Result<(Vec<user_token::Model>, u64), DatabaseError> {
        // Setup paginator
        let paginator = UserToken::find()
            .order_by_asc(user_token::Column::Id)
            .paginate(&self.0, per_page);
        let num_pages = paginator.num_pages().await.map_err(to_db_error)?;
        // Fetch paginator tokens
        paginator.fetch_page(page).await.map_err(to_db_error).map(|p| (p, num_pages))
    }
    pub async fn get_user_tokens_by_user_id(&self, user_id: u64) -> Result<Vec<user_token::Model>, DatabaseError> {
        UserToken::find()
            .filter(user_token::Column::UserId.eq(user_id))
            .order_by_asc(user_token::Column::Id)
            .all(&self.0)
            .await.map_err(to_db_error)
    }
    pub async fn get_user_token_by_id(&self, id: u64) -> Result<user_token::Model, DatabaseError> {
        Ok(UserToken::find_by_id(id as i32).one(&self.0).await.map_err(to_db_error)?.ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("UserToken not found"))})?)
    }
    pub async fn get_user_token(&self, token: &str) -> Result<user_token::Model, DatabaseError> {
        Ok(UserToken::find().filter(user_token::Column::Token.contains(token)).one(&self.0).await.map_err(to_db_error)?.ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("UserToken not found"))})?)
    }
    pub async fn create_user_token(&self, user_token: user_token::ActiveModel) -> Result<user_token::ActiveModel, DatabaseError> {
        let user_token = user_token.try_into_model().expect("Can't into model");
        user_token::ActiveModel {
            user_id: Set(user_token.user_id.to_owned()),
            token: Set(user_token.token.to_owned()),
            note: Set(user_token.note.to_owned()),
            enabled: Set(user_token.enabled.to_owned()),
            expiration_time: Set(user_token.expiration_time.to_owned()),
            creation_time: Set(Local::now().naive_local().to_owned()),
            last_edit_time: Set(None),
            last_usage_time: Set(None),
            version: Set(1),
            ..Default::default()
        }
        .save(&self.0)
        .await.map_err(to_db_error)
    }
    pub async fn update_user_token(&self, id: u64, user_token: user_token::ActiveModel) -> Result<user_token::Model, DatabaseError> {
        let current_user_token: user_token::ActiveModel = UserToken::find_by_id(id as i32)
            .one(&self.0)
            .await.map_err(to_db_error)?
            .ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("UserToken not found"))})
            .map(Into::into)?;
        let user_token = user_token.try_into_model().expect("Can't into model");
        user_token::ActiveModel {
            id: current_user_token.id,
            user_id: current_user_token.user_id,
            token: current_user_token.token,
            note: Set(user_token.note.to_owned()),                               // Can be updated
            enabled: Set(user_token.enabled.to_owned()),                         // Can be updated
            expiration_time: Set(user_token.expiration_time.to_owned()),         // Can be updated
            creation_time: current_user_token.creation_time,
            last_edit_time: Set(Some(Local::now().naive_local().to_owned())),
            last_usage_time: current_user_token.last_usage_time,
            version: Set(user_token.version.to_owned()),                         // Can be updated
        }
        .update(&self.0)
        .await.map_err(to_db_error)
    }
    pub async fn delete_user_token(&self, token: &str) -> Result<(), DatabaseError> {
        let user_token: user_token::ActiveModel = UserToken::find()
            .filter(user_token::Column::Token.contains(token))
            .one(&self.0)
            .await.map_err(to_db_error)?
            .ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("UserToken not found"))})
            .map(Into::into)?;

        user_token.delete(&self.0).await.map_err(to_db_error)?;
        Ok(())
    }
    pub async fn update_last_token_usage_time(&self, name: &str) -> Result<user_token::Model, DatabaseError> {
        let mut user_token: user_token::ActiveModel = UserToken::find()
            .filter(user_token::Column::Token.contains(name))
            .one(&self.0)
            .await.map_err(to_db_error)?
            .ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("UserToken not found"))})
            .map(Into::into)?;

        user_token.last_usage_time = Set(Some(Local::now().naive_local().to_owned()));

        user_token.update(&self.0).await.map_err(to_db_error)
    }
    // Snapshot
    pub async fn get_snapshots_count(&self) -> Result<u64, DatabaseError> {
        Snapshot::find().count(&self.0).await.map_err(to_db_error)
    }
    pub async fn get_snapshots_in_page(&self, page: u64, per_page: u64) -> Result<(Vec<snapshot::Model>, u64), DatabaseError> {
        // Setup paginator
        let paginator = Snapshot::find()
            .order_by_asc(snapshot::Column::Id)
            .paginate(&self.0, per_page);
        let num_pages = paginator.num_pages().await.map_err(to_db_error)?;
        // Fetch paginator users
        paginator.fetch_page(page - 1).await.map_err(to_db_error).map(|p| (p, num_pages))
    }
    pub async fn create_snapshot(&self, snapshot: snapshot::ActiveModel) -> Result<snapshot::ActiveModel, DatabaseError> {
        let snapshot = snapshot.try_into_model().expect("Can't into model");
        snapshot::ActiveModel {
            creation_time: Set(Local::now().naive_local().to_owned()),
            resource_type: Set(snapshot.resource_type.to_owned()),
            operation: Set(snapshot.operation.to_owned()),
            user_id: Set(snapshot.user_id.to_owned()),
            data: Set(snapshot.data.to_owned()),
            resource_name: Set(snapshot.resource_name.to_owned()),
            resource_pkey: Set(snapshot.resource_pkey.to_owned()),
            ..Default::default()
        }
        .save(&self.0)
        .await.map_err(to_db_error)
    }
    pub async fn delete_snapshot(&self, id: u64) -> Result<(), DatabaseError> {
        let snapshot: snapshot::ActiveModel = Snapshot::find_by_id(id as i32)
            .one(&self.0)
            .await.map_err(to_db_error)?
            .ok_or_else(|| {DatabaseError::from(anyhow::anyhow!("Snapshot not found"))})
            .map(Into::into)?;

        snapshot.delete(&self.0).await.map_err(to_db_error)?;
        Ok(())
    }
}