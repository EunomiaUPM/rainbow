pub mod migrations;
pub mod entities;

use anyhow::bail;
use sea_orm::{Database, DatabaseConnection};

pub async fn db_connection() -> anyhow::Result<DatabaseConnection> {
    let db = Database::connect("postgres://ds-protocol-provider:ds-protocol-provider@localhost:5433/ds-protocol-provider").await;
    match db {
        Ok(db) => Ok(db),
        Err(e) => bail!(e),
    }
}