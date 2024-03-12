use ::entity::{user_token, user_token::Entity as UserToken};
use chrono::Local;
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_user_tokens_in_page(
        db: &DbConn,
        page: u64,
        tokens_per_page: u64,
    ) -> Result<(Vec<user_token::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = UserToken::find()
            .order_by_asc(user_token::Column::Id)
            .paginate(db, tokens_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginator tokens
        paginator.fetch_page(page).await.map(|p| (p, num_pages))
    }

    pub async fn count_tokens(db: &DbConn) -> Result<u64, DbErr> {
        UserToken::find().count(db).await
    }

    pub async fn find_token_by_id(db: &DbConn, id: i32) -> Result<Option<user_token::Model>, DbErr> {
        UserToken::find_by_id(id).one(db).await
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn create_token(
        db: &DbConn,
        form_data: user_token::Model,
    ) -> Result<user_token::ActiveModel, DbErr> {
        user_token::ActiveModel {
            user_id: Set(form_data.user_id.to_owned()),
            token: Set(form_data.token.to_owned()),
            note: Set(form_data.note.to_owned()),
            enabled: Set(form_data.enabled.to_owned()),
            expiration_time: Set(form_data.expiration_time.to_owned()),
            creation_time: Set(Local::now().naive_local().to_owned()),
            last_edit_time: Set(None),
            last_usage_time: Set(None),
            version: Set(1),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_token_by_id(
        db: &DbConn,
        id: i32,
        form_data: user_token::Model,
    ) -> Result<user_token::Model, DbErr> {
        let user_token: user_token::ActiveModel = UserToken::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user_token.".to_owned()))
            .map(Into::into)?;
        user_token::ActiveModel {
            id: user_token.id,
            user_id: user_token.user_id,
            token: user_token.token,
            note: Set(form_data.note.to_owned()),                               // Can be updated
            enabled: Set(form_data.enabled.to_owned()),                         // Can be updated
            expiration_time: Set(form_data.expiration_time.to_owned()),         // Can be updated
            creation_time: user_token.creation_time,
            last_edit_time: Set(Some(Local::now().naive_local().to_owned())),
            last_usage_time: user_token.last_usage_time,
            version: Set(form_data.version.to_owned()),                         // Can be updated
        }
        .update(db)
        .await
    }

    pub async fn delete_token(db: &DbConn, token: String) -> Result<DeleteResult, DbErr> {
        let user_token: user_token::ActiveModel = UserToken::find()
            .filter(user_token::Column::Token.contains(token))
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user_token.".to_owned()))
            .map(Into::into)?;

        user_token.delete(db).await
    }

    // pub async fn delete_all_user_tokens(db: &DbConn) -> Result<DeleteResult, DbErr> {
    //     UserToken::delete_many().exec(db).await
    // }
}