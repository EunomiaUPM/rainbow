use sea_orm_migration::prelude::*;

mod m20241111_000001_catalog;
mod m20241111_000002_dataset;
mod m20241111_000003_distribution;
mod m20241111_000004_dataservice;
pub mod m20241111_000005_odrl_offers;
mod m20241111_000006_keywords_themes;

pub fn get_catalog_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20241111_000001_catalog::Migration),
        Box::new(m20241111_000002_dataset::Migration),
        Box::new(m20241111_000003_distribution::Migration),
        Box::new(m20241111_000004_dataservice::Migration),
        Box::new(m20241111_000005_odrl_offers::Migration),
        // Box::new(m20241111_000006_keywords_themes::Migration)
    ]
}
pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        get_catalog_migrations()
    }
}