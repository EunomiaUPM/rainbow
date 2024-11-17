pub mod migrations;
pub mod entities;

use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

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


pub async fn db_connection() -> anyhow::Result<DatabaseConnection> {
    let db = Database::connect("postgres://ds-protocol-provider:ds-protocol-provider@localhost:5435/ds-protocol-provider").await;
    match db {
        Ok(db) => Ok(db),
        Err(e) => panic!("Database connection error: {}", e.to_string()),
    }
}