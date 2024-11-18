use crate::config::config::get_local_database_url;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;
// use rainbow_common::config::config::GLOBAL_CONFIG;

pub static DB_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();
pub async fn get_db_connection() -> &'static DatabaseConnection {
    println!("{:?}", get_local_database_url());
    DB_CONNECTION.get_or_init(|| async {
        let db_url = get_local_database_url().unwrap();
        let db = Database::connect(db_url).await;
        match db {
            Ok(db) => {
                println!("Database connection successfully established");
                db
            }
            Err(e) => panic!("Database connection error: {}", e.to_string()),
        }
    }).await
}
