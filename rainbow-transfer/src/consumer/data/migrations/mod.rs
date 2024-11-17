use sea_orm_migration::prelude::*;

mod m20241117_000001_transfer_callbacks;

pub fn get_transfer_consumer_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20241117_000001_transfer_callbacks::Migration),
    ]
}

pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        get_transfer_consumer_migrations()
    }
}