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
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::UserId).integer())
                    .col(ColumnDef::new(Post::CreationTime).timestamp().not_null())
                    .col(ColumnDef::new(Post::LastEditTime).timestamp())
                    .col(ColumnDef::new(Post::Safety).string_len(32).not_null())
                    .col(ColumnDef::new(Post::Type).string_len(32).not_null())
                    .col(ColumnDef::new(Post::Checksum).string_len(64).not_null())
                    .col(ColumnDef::new(Post::Source).string_len(2048))
                    .col(ColumnDef::new(Post::FileSize).big_integer())
                    .col(ColumnDef::new(Post::ImageWidth).integer())
                    .col(ColumnDef::new(Post::ImageHeight).integer())
                    .col(ColumnDef::new(Post::MimeType).string_len(32).not_null())
                    .col(ColumnDef::new(Post::Version).integer().not_null())
                    .col(ColumnDef::new(Post::Flags).string_len(32))
                    .col(ColumnDef::new(Post::ChecksumMD5).string_len(32))
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_posts_userid")
                            .from(Post::Table, Post::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "creation_time")]
    CreationTime,
    #[sea_orm(iden = "last_edit_time")]
    LastEditTime,
    Safety,
    Type,
    Checksum,
    Source,
    #[sea_orm(iden = "file_size")]
    FileSize,
    #[sea_orm(iden = "image_width")]
    ImageWidth,
    #[sea_orm(iden = "image_height")]
    ImageHeight,
    #[sea_orm(iden = "mime-type")]
    MimeType,
    Version,
    Flags,
    #[sea_orm(iden = "checksum_md5")]
    ChecksumMD5,
}
