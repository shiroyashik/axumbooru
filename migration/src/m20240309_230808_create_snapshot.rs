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
                    .table(Snapshot::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Snapshot::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Snapshot::CreationTime).timestamp().not_null())
                    .col(ColumnDef::new(Snapshot::ResourceType).string_len(32).not_null())
                    .col(ColumnDef::new(Snapshot::Operation).string_len(16).not_null())
                    .col(ColumnDef::new(Snapshot::UserId).integer())
                    .col(ColumnDef::new(Snapshot::Data).binary())
                    .col(ColumnDef::new(Snapshot::ResourceName).string_len(128).not_null())
                    .col(ColumnDef::new(Snapshot::ResourcePkey).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_snapshot_userid")
                            .from(Snapshot::Table, Snapshot::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Snapshot::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Snapshot {
    Table,
    Id,
    #[sea_orm(iden = "creation_time")]
    CreationTime,
    #[sea_orm(iden = "resource_type")]
    ResourceType,
    Operation,
    #[sea_orm(iden = "user_id")]
    UserId,
    Data,
    #[sea_orm(iden = "resource_name")]
    ResourceName,
    #[sea_orm(iden = "resource_pkey")]
    ResourcePkey,
}
