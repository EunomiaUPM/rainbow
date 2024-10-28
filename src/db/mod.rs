use crate::setup::config::get_consumer_database_url;
use crate::setup::config::get_provider_database_url;
use crate::setup::config::{ConfigRoles, GLOBAL_CONFIG};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use duckdb::DuckdbConnectionManager;

///
/// ⚠️Warning!
/// NOTE of building
/// It could happen in some systems that an error related with -ld,
/// with a message such as:
/// ld: library not found for -lpq when build rust in macOS
/// It's a problem of linking homebrew and libpq with postgres in macOS
/// Likewise could happen in ubuntu, so: sudo apt install libpq-dev
/// Solution here:
/// https://stackoverflow.com/questions/70313347/ld-library-not-found-for-lpq-when-build-rust-in-macos
///
pub fn get_db_relational_connection_r2d2() -> Pool<ConnectionManager<PgConnection>> {
    // TODO refactor this for working with different databases type or none...
    let db_url = match GLOBAL_CONFIG.get().unwrap().role {
        ConfigRoles::Provider => get_provider_database_url(),
        ConfigRoles::Consumer => get_consumer_database_url(),
        _ => panic!("Unsupported dataspace role"),
    }.unwrap();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .max_size(10)
        .test_on_check_out(true)
        .build(manager)
        .expect("Database pool initialization failed")
}

pub fn get_db_memory_connection_r2d2() -> Pool<DuckdbConnectionManager> {
    let conn = DuckdbConnectionManager::file("db").unwrap();
    // let conn = DuckdbConnectionManager::memory.unwrap();
    Pool::builder()
        .max_size(10)
        .test_on_check_out(true)
        .build(conn)
        .expect("Database pool initialization failed")
}