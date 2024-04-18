use crate::db::schemas::{user, user::Entity as User};
use chrono::{Local, NaiveDateTime as DateTime};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_users_in_page(
        db: &DbConn,
        page: u64,
        users_per_page: u64,
    ) -> Result<(Vec<user::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = User::find()
            .order_by_asc(user::Column::Id)
            .paginate(db, users_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginator users
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn find_user_by_name(db: &DbConn, name: &str) -> Result<Option<user::Model>, DbErr> {
        User::find().filter(user::Column::Name.contains(name)).one(db).await
    }

    // Можно использовать как шаблон для запроса из базы данных с фильтром
    pub async fn find_user_credentials_by_name(db: &DbConn, name: &str) -> Result<Option<user::Model>, DbErr> {
        User::find()
        .filter(user::Column::Name.contains(name))
        .select_only()
        .columns([user::Column::Id, user::Column::Name, user::Column::PasswordHash])
        .one(db).await
    }

    pub async fn find_user_credentials_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id)
        .select_only()
        .columns([user::Column::Id, user::Column::Name, user::Column::PasswordHash])
        .one(db).await
    }

    pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }
}

pub struct Mutation {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub password_hash: Option<String>,
    pub password_salt: Option<String>,
    pub email: Option<String>,
    pub rank: Option<String>,
    pub creation_time: Option<DateTime>,
    pub last_login_time: Option<DateTime>,
    pub avatar_style: Option<String>,
    pub version: Option<i32>,
    pub password_revision: Option<i16>,
}

impl Mutation {
    pub async fn create_user(
        db: &DbConn,
        form_data: Self,
    ) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
            name: Set(form_data.name.unwrap()),
            password_hash: Set(form_data.password_hash.unwrap()),
            password_salt: Set(form_data.password_salt.to_owned()),
            email: Set(form_data.email.to_owned()),
            rank: Set(form_data.rank.unwrap()),
            creation_time: Set(Local::now().naive_local().to_owned()),
            avatar_style: Set(form_data.avatar_style.unwrap()),
            version: Set(1),
            password_revision: Set(3),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_user_by_id(
        db: &DbConn,
        id: i32,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;
        user::ActiveModel {
            id: user.id,
            name: user.name,
            password_hash: Set(form_data.password_hash.to_owned()),
            password_salt: Set(form_data.password_salt.to_owned()),
            email: Set(form_data.email.to_owned()),
            rank: Set(form_data.rank.to_owned()),
            creation_time: user.creation_time,
            last_login_time: user.last_login_time,
            avatar_style: Set(form_data.avatar_style.to_owned()),
            version: Set(form_data.version.to_owned()),
            password_revision: Set(form_data.password_revision.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn update_last_login_time_by_name(
        db: &DbConn,
        name: &str,
    ) -> Result<user::Model, DbErr> {
        let mut user: user::ActiveModel = User::find()
            .filter(user::Column::Name.contains(name))
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user.last_login_time = Set(Some(Local::now().naive_local().to_owned()));

        user.update(db).await
    }

    pub async fn delete_user(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user.delete(db).await
    }

    // pub async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
    //     User::delete_many().exec(db).await
    // }
}
