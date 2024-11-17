use sea_orm_migration::prelude::*;
pub mod m20241116_000003_contract_agreements;

pub fn get_contracts_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![Box::new(m20241116_000003_contract_agreements::Migration)]
}

pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        get_contracts_migrations()
    }
}
