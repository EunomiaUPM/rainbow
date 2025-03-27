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

use clap::{Parser, Subcommand};
use sea_orm_migration::MigratorTrait;

pub mod config;
pub mod application;
pub mod db_migrations;
pub mod cmd;
// #[derive(Parser, Debug)]
// #[command(name = "Dataspace protocol catalog")]
// #[command(version = "0.1")]
// #[command(about = "Dataspace protocol catalog", long_about = "Dataspace protocol catalog")]
// struct CatalogCli {
//     #[command(subcommand)]
//     command: DataSpaceCatalogCommands,
// }
// #[derive(Subcommand, Debug)]
// enum DataSpaceCatalogCommands {
//     #[command(about = "Migrate database")]
//     MigrateDatabase,
//     #[command(about = "Start the catalog servers")]
//     Start,
// }
//
// pub async fn init_command_line() -> anyhow::Result<()> {
//     let cli = CatalogCli::parse();
//     match &cli.command {
//         DataSpaceCatalogCommands::MigrateDatabase => {
//             let db_connection = get_db_connection().await;
//             Migrator::refresh(db_connection).await?;
//             Ok(())
//         }
//         DataSpaceCatalogCommands::Start => {
//             let server_message = "Starting provider server in 0.0.0.0:1234".to_string();
//             info!("{}", server_message);
//             let listener = TcpListener::bind("0.0.0.0:1234").await?;
//             let router = Router::new()
//                 .merge(catalog_router().await?)
//                 .merge(catalog_api_router().await?)
//                 .merge(catalog_policies_api_router().await?);
//             let _ = serve(listener, router).await;
//             Ok(())
//         }
//     }
// }
