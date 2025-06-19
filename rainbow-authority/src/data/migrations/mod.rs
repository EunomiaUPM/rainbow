use sea_orm::prelude::*;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub mod m20250529_000001_vc_requests;

pub fn get_authority_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20250529_000001_vc_requests::Migration),
    ]
}

pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        get_authority_migrations()
    }
}
