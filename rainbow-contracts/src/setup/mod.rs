pub mod databases;

use crate::data::migrations::Migrator;
use crate::setup::databases::get_db_connection;
use axum::{serve, Router};
use clap::{Parser, Subcommand};
use sea_orm_migration::MigratorTrait;
use tokio::net::TcpListener;
use tracing::info;

// TODO export to lib to be interoperable with modules
// TODO parse env

#[derive(Parser, Debug)]
#[command(name = "Dataspace protocol contracts")]
#[command(version = "0.1")]
#[command(about = "Dataspace protocol contracts", long_about = "Dataspace protocol contracts")]
struct CatalogCli {
    #[command(subcommand)]
    command: DataSpaceCatalogCommands,
}
#[derive(Subcommand, Debug)]
enum DataSpaceCatalogCommands {
    #[command(about = "Migrate database")]
    MigrateDatabase,
    #[command(about = "Start the contracts servers")]
    Start,
}

pub async fn init_command_line() -> anyhow::Result<()> {
    let cli = CatalogCli::parse();
    match &cli.command {
        DataSpaceCatalogCommands::MigrateDatabase => {
            let db_connection = get_db_connection().await;
            Migrator::refresh(db_connection).await?;
            Ok(())
        }
        DataSpaceCatalogCommands::Start => {
            let server_message = "Starting provider server in 0.0.0.0:8001".to_string();
            info!("{}", server_message);
            let listener = TcpListener::bind("0.0.0.0:8001").await?;
            let router = Router::new();
            let _ = serve(listener, router).await;
            Ok(())
        }
    }
}
