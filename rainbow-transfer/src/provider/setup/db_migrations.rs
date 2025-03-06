use crate::provider::setup::config::TransferProviderApplicationConfig;
use rainbow_db::dataplane::migrations::get_dataplane_migrations;
use rainbow_db::transfer_provider::migrations::get_transfer_provider_migrations;
use sea_orm::Database;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct TransferProviderMigration;

impl MigratorTrait for TransferProviderMigration {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations: Vec<Box<dyn MigrationTrait>> = vec![];
        let mut provider_migrations = get_transfer_provider_migrations();
        let mut dataplane_migrations = get_dataplane_migrations();

        migrations.append(&mut provider_migrations);
        migrations.append(&mut dataplane_migrations);
        migrations
    }
}

impl TransferProviderMigration {
    pub async fn run(config: &TransferProviderApplicationConfig<'static>) -> anyhow::Result<()> {
        // db_connection
        let db_url = config.get_full_db_url();
        let db_connection = Database::connect(db_url).await.expect("Database can't connect");
        // run migration
        Self::refresh(&db_connection).await?;
        Ok(())
    }
}