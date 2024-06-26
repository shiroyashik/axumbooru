//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "snapshot")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub creation_time: DateTime,
    pub resource_type: String,
    pub operation: String,
    pub user_id: Option<i32>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub data: Option<Vec<u8>>,
    pub resource_name: String,
    pub resource_pkey: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
