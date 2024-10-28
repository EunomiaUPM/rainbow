use crate::db::get_db_relational_connection_r2d2;
use crate::setup::config::get_provider_database_url;
use crate::setup::config::GLOBAL_CONFIG;
use diesel::dsl::{exists, select};
use diesel::migration::MigrationSource;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_types::Text;
use diesel::{PgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use duckdb::{Config, DuckdbConnectionManager};
use log::info;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tracing::error;

pub const PROVIDER_MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./src/db/provider_migrations");
pub const CONSUMER_MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./src/db/consumer_migrations");

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
    info!("Connecting to memory database");

    let mut file = match role.as_str() {
        "provider" => File::open("./src/db/memory_migrations/setup_provider.sql"),
        "consumer" => File::open("./src/db/memory_migrations/setup_consumer.sql"),
        _ => panic!("Unsupported role: {}", role),
    }.await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    // let manager = DuckdbConnectionManager::memory()?;
    let manager = DuckdbConnectionManager::file("db")?;
    let db_connection = Pool::builder().max_size(1).build(manager);
    if db_connection.is_err() {
        error!("Could not connect to database");
        return Err(anyhow::anyhow!(
            "Database connection could not be established"
        ));
    }

    let conn = db_connection?.get()?;
    conn.execute_batch(contents.as_str()).expect("TODO: panic message");
    Ok(())
}

pub async fn setup_database_mongo(role: String) -> anyhow::Result<()> {
    todo!()
}

pub async fn setup_database_postgres(role: String) -> anyhow::Result<()> {
    info!("Connecting to database");
    let db_connection_url = get_provider_database_url()?;
    let db_name = GLOBAL_CONFIG.get().unwrap().db_database.clone();
    info!("{}", db_connection_url);

    let migrations = match role.as_str() {
        "provider" => PROVIDER_MIGRATIONS,
        "consumer" => CONSUMER_MIGRATIONS,
        _ => panic!("Unsupported role: {}", role),
    };
    let manager = ConnectionManager::<PgConnection>::new(db_connection_url);
    let db_connection = Pool::builder().max_size(1).build(manager);

    // Check if connection is ok
    if db_connection.is_err() {
        error!("Could not connect to database");
        error!("{}", db_connection.unwrap_err());
        return Err(anyhow::anyhow!(
            "Database connection could not be established"
        ));
    }

    // Migrate stuff if needed
    let mut db_connection = db_connection?.get()?;
    let migration = db_connection.run_pending_migrations(migrations);
    if migration.is_err() {
        error!("{}", migration.unwrap_err());
        return Err(anyhow::anyhow!("Migrations could not be completed"));
    }

    Ok(())
}
