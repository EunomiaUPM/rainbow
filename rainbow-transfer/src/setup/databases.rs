use log::info;
use rainbow_common::config::config::GLOBAL_CONFIG;
use rainbow_common::config::database::get_db_connection;
use rainbow_db::transfer_consumer::migrations::Migrator as ConsumerMigrator;
use rainbow_db::transfer_provider::migrations::Migrator as ProviderMigrator;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::OnceCell;
use tracing::error;

pub async fn setup_database(role: String) -> anyhow::Result<()> {
    match GLOBAL_CONFIG.get().unwrap().db_type.as_str() {
        "postgres" => setup_database_postgres(role).await?,
        // "memory" => setup_database_memory(role).await?,
        _ => panic!("Database supplied doesn't exist"),
    }
    Ok(())
}

pub async fn setup_database_memory(role: String) -> anyhow::Result<()> {
    todo!()
}

pub async fn setup_database_mongo(role: String) -> anyhow::Result<()> {
    todo!()
}

pub async fn setup_database_postgres(role: String) -> anyhow::Result<()> {
    info!("Connecting to database");
    let db_connection = get_db_connection().await;
    let db_name = GLOBAL_CONFIG.get().unwrap().db_database.clone();
    info!("{}", db_name);

    match role.as_str() {
        "provider" => {
            ProviderMigrator::refresh(db_connection).await?;
            Ok(())
        }
        "consumer" => {
            ConsumerMigrator::refresh(db_connection).await?;
            Ok(())
        }
        _ => panic!("Unsupported role: {}", role),
    }
}
