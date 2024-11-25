use rainbow_catalog::data::migrations::get_catalog_migrations;
use rainbow_common::config::database::get_db_connection;
use rainbow_dataplane::data::migrations::get_dataplane_migrations;
use rainbow_transfer::consumer::data::migrations::get_transfer_consumer_migrations;
use rainbow_transfer::provider::data::migrations::get_transfer_provider_migrations;
use sea_orm::prelude::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct ProviderMigrator;
#[async_trait::async_trait]
impl MigratorTrait for ProviderMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations = vec![];
        let mut provider_migrations = get_transfer_provider_migrations();
        let mut catalog_migrations = get_catalog_migrations();
        let mut dataplane_migrations = get_dataplane_migrations();
        // let mut contract_migrations = get_contracts_migrations();
        migrations.append(&mut provider_migrations);
        migrations.append(&mut catalog_migrations);
        migrations.append(&mut dataplane_migrations);
        // migrations.append(&mut contract_migrations);
        migrations
    }
}

pub struct ConsumerMigrator;
#[async_trait::async_trait]
impl MigratorTrait for ConsumerMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        // TODO migrations all in a same workspace for dealing with cross compiling fks
        let mut migrations = vec![];
        let mut consumer_migrations = get_transfer_consumer_migrations();
        let mut dataplane_migrations = get_dataplane_migrations();
        migrations.append(&mut consumer_migrations);
        migrations.append(&mut dataplane_migrations);
        migrations
    }
}

pub async fn migrate_provider_db() -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    ProviderMigrator::refresh(db_connection).await?;
    Ok(())
}

pub async fn migrate_consumer_db() -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    ConsumerMigrator::refresh(db_connection).await?;
    Ok(())
}