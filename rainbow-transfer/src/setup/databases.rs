use crate::consumer::data::migrations::Migrator as ConsumerMigrator;
use crate::provider::data::migrations::Migrator as ProviderMigrator;
use crate::setup::config::get_provider_database_url;
use crate::setup::config::GLOBAL_CONFIG;
use log::info;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::OnceCell;
use tracing::error;

pub static DB_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();
pub async fn get_db_connection() -> &'static DatabaseConnection {
    DB_CONNECTION.get_or_init(|| async {
        let db = Database::connect("postgres://ds-protocol-provider:ds-protocol-provider@localhost:5435/ds-protocol-provider").await;
        match db {
            Ok(db) => db,
            Err(e) => panic!("Database connection error: {}", e.to_string()),
        }
    }).await
}

pub async fn setup_database(role: String) -> anyhow::Result<()> {
    match GLOBAL_CONFIG.get().unwrap().db_type.as_str() {
        "postgres" => setup_database_postgres(role).await?,
        "memory" => setup_database_memory(role).await?,
        "mongo" => setup_database_mongo(role).await?,
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
