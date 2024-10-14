use crate::config::GLOBAL_CONFIG;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

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
pub fn get_db_connection() -> Pool<ConnectionManager<PgConnection>> {
    // TODO refactor this for working with different databases type or none...
    // TODO change the way of doing provider_migrations...

    let db_url = GLOBAL_CONFIG.get().unwrap().db_url.clone();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .max_size(10)
        .test_on_check_out(true)
        .build(manager)
        .expect("Database pool initialization failed")
}
