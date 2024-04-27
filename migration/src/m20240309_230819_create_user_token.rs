use sea_orm_migration::prelude::*;

use crate::m20240225_224934_create_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(UserToken::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserToken::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserToken::UserId).integer().not_null())
                    .col(ColumnDef::new(UserToken::Token).string_len(36).not_null())
                    .col(ColumnDef::new(UserToken::Note).string_len(128))
                    .col(ColumnDef::new(UserToken::Enabled).boolean().not_null())
                    .col(ColumnDef::new(UserToken::ExpirationTime).timestamp())
                    .col(ColumnDef::new(UserToken::CreationTime).timestamp().not_null())
                    .col(ColumnDef::new(UserToken::LastEditTime).timestamp())
                    .col(ColumnDef::new(UserToken::LastUsageTime).timestamp())
                    .col(ColumnDef::new(UserToken::Version).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_user_token_userid")
                            .from(UserToken::Table, UserToken::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(UserToken::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserToken {
    Table,
    Id,
    #[sea_orm(iden = "user_id")]
    UserId,
    Token,
    Note,
    Enabled,
    #[sea_orm(iden = "expiration_time")]
    ExpirationTime,
    #[sea_orm(iden = "creation_time")]
    CreationTime,
    #[sea_orm(iden = "last_edit_time")]
    LastEditTime,
    #[sea_orm(iden = "last_usage_time")]
    LastUsageTime,
    Version,
}
