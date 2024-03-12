use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Name)
                            .string_len(50)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::PasswordHash).string_len(128).not_null())
                    .col(ColumnDef::new(User::PasswordSalt).string_len(32))
                    .col(ColumnDef::new(User::Email).string_len(64))
                    .col(ColumnDef::new(User::Rank).string_len(32).not_null())
                    .col(ColumnDef::new(User::CreationTime).timestamp().not_null())
                    .col(ColumnDef::new(User::LastLoginTime).timestamp())
                    .col(ColumnDef::new(User::AvatarStyle).string_len(32).not_null())
                    .col(ColumnDef::new(User::Version).integer().not_null())
                    .col(ColumnDef::new(User::PasswordRevision).small_integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(super) enum User {
    Table,
    Id,
    Name,
    #[sea_orm(iden = "password_hash")]
    PasswordHash,
    #[sea_orm(iden = "password_salt")]
    PasswordSalt,
    Email,
    Rank,
    #[sea_orm(iden = "creation_time")]
    CreationTime,
    #[sea_orm(iden = "last_login_time")]
    LastLoginTime,
    #[sea_orm(iden = "avatar_style")]
    AvatarStyle,
    Version,
    #[sea_orm(iden = "password_revision")]
    PasswordRevision,
}
