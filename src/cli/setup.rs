use crate::config::provider::get_provider_database_url;
use crate::config::GLOBAL_CONFIG;
use crate::db::get_db_connection;
use diesel::dsl::{exists, select};
use diesel::migration::MigrationSource;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_types::Text;
use diesel::{PgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use tracing::error;

pub const PROVIDER_MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./src/db/provider_migrations");
pub const CONSUMER_MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./src/db/consumer_migrations");

pub async fn setup_database(role: String) -> anyhow::Result<()> {
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
