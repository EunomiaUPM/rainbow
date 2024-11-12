use axum::serve;
use rainbow_catalog;
use rainbow_catalog::data::db_connection;
use rainbow_catalog::data::entities::catalog::Entity as CatalogEntity;
use rainbow_catalog::data::entities::dataset::Entity as DatasetEntity;
use rainbow_catalog::data::migrations::Migrator;
use rainbow_catalog::http::api;
use rainbow_catalog::http::api::catalog_router;
use sea_orm::EntityTrait;
use sea_orm_migration::MigratorTrait;
use tokio::net::TcpListener;
use tracing::info;

const INFO: &str = r"
----------
 ____    __    ____  _  _  ____  _____  _    _
(  _ \  /__\  (_  _)( \( )(  _ \(  _  )( \/\/ )
 )   / /(__)\  _)(_  )  (  ) _ < )(_)(  )    (
(_)\_)(__)(__)(____)(_)\_)(____/(_____)(__/\__)

Starting Rainbow Catalog Server 🌈🌈
UPM Dataspace protocol implementation
Show some love on https://github.com/ging/rainbow
----------

";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().init();

    info!("{}", INFO);
    let db_connection = db_connection().await.unwrap();
    // Migrator::refresh(&db_connection).await.unwrap();

    let server_message = "Starting provider server in 0.0.0.0:8000".to_string();
    info!("{}", server_message);
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    serve(listener, catalog_router().await.unwrap()).await;
}
