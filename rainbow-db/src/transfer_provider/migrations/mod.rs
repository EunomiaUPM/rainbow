use sea_orm_migration::prelude::*;

mod m20241116_000001_transfer_process;
mod m20241116_000002_transfer_messages;
mod m20241116_000003_contract_agreements;

pub fn get_transfer_provider_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20241116_000001_transfer_process::Migration),
        Box::new(m20241116_000002_transfer_messages::Migration),
        Box::new(m20241116_000003_contract_agreements::Migration),
    ]
}

pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        get_transfer_provider_migrations()
    }
}