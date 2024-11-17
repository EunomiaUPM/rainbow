#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use rainbow_transfer::provider::data::get_db_connection;
use rainbow_transfer::provider::data::migrations::Migrator;
use sea_orm_migration::MigratorTrait;
use tracing::info;

const INFO: &str = r"
----------
 ____    __    ____  _  _  ____  _____  _    _
(  _ \  /__\  (_  _)( \( )(  _ \(  _  )( \/\/ )
 )   / /(__)\  _)(_  )  (  ) _ < )(_)(  )    (
(_)\_)(__)(__)(____)(_)\_)(____/(_____)(__/\__)

Starting Rainbow Transfer Server 🌈🌈
UPM Dataspace protocol implementation
Show some love on https://github.com/ging/rainbow
----------

";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().init();
    info!("{}", INFO);

    let db_connection = get_db_connection().await;
    // Migrator::down(db_connection, None).await.unwrap();
    Migrator::refresh(db_connection).await.unwrap();
}
