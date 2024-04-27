pub use sea_orm_migration::prelude::*;

mod m20240225_224934_create_user;
mod m20240227_020126_create_post;
mod m20240309_230819_create_user_token;
mod m20240309_230808_create_snapshot;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240225_224934_create_user::Migration),
            Box::new(m20240227_020126_create_post::Migration),
            Box::new(m20240309_230808_create_snapshot::Migration),
            Box::new(m20240309_230819_create_user_token::Migration),
        ]
    }
}
