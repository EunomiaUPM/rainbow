use crate::data::db_connection;
use crate::data::migrations::Migrator;
use crate::http::ll_api::catalog_router;
use axum::{serve, Router};
use clap::{Parser, Subcommand};
use sea_orm_migration::MigratorTrait;
use tokio::net::TcpListener;
use tracing::info;
use crate::http::hl_api::catalog_api_router;
use crate::http::policies_api::catalog_policies_api_router;

// TODO export to lib to be interoperable with modules
// TODO parse env

#[derive(Parser, Debug)]
#[command(name = "Dataspace protocol catalog")]
#[command(version = "0.1")]
#[command(about = "Dataspace protocol catalog", long_about = "Dataspace protocol catalog")]
struct CatalogCli {
    #[command(subcommand)]
    command: DataSpaceCatalogCommands,
}
#[derive(Subcommand, Debug)]
enum DataSpaceCatalogCommands {
    #[command(about = "Migrate database")]
    MigrateDatabase,
    #[command(about = "Start the catalog servers")]
    Start,
}

pub async fn init_command_line() -> anyhow::Result<()> {
    let cli = CatalogCli::parse();
    match &cli.command {
        DataSpaceCatalogCommands::MigrateDatabase => {
            let db_connection = db_connection().await.unwrap();
            Migrator::refresh(&db_connection).await?;
            Ok(())
        }
        DataSpaceCatalogCommands::Start => {
            let server_message = "Starting provider server in 0.0.0.0:8000".to_string();
            info!("{}", server_message);
            let listener = TcpListener::bind("0.0.0.0:8000").await?;
            let router = Router::new()
                .merge(catalog_router().await?)
                .merge(catalog_api_router().await?)
                .merge(catalog_policies_api_router().await?);
            serve(listener, router).await;
            Ok(())
        }
    }
}