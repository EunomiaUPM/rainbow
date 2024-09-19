use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;

pub mod models;
pub mod schema;

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

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .max_size(10)
        .test_on_check_out(true)
        .build(manager)
        .expect("aaa")
}
