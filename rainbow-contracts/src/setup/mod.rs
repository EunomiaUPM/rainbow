/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use axum::{serve, Router};
use clap::{Parser, Subcommand};
use rainbow_common::config::database::get_db_connection;
use rainbow_db::contracts_provider::migrations::Migrator;
use sea_orm_migration::MigratorTrait;
use tokio::net::TcpListener;
use tracing::info;

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
