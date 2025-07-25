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
use rainbow_catalog::provider::setup::application::CatalogApplication;
use rainbow_catalog::provider::setup::db_migrations::CatalogMigration;
use rainbow_common::config::env_extraction::EnvExtraction;
use std::cmp::PartialEq;
use tracing::debug;

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Datahub Catalog Server")]
#[command(version = "0.2")]
struct CatalogCli {
    #[command(subcommand)]
    command: CatalogCliCommands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum CatalogCliCommands {
    Start(CatalogCliArgs),
    Setup(CatalogCliArgs),
}

#[derive(Parser, Debug, PartialEq)]
pub struct CatalogCliArgs {
    #[arg(short, long)]
    env_file: Option<String>,
}

pub struct CatalogCommands;

impl EnvExtraction for CatalogCommands {}

impl CatalogCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = CatalogCli::parse();

        // run scripts
        match cli.command {
            CatalogCliCommands::Start(args) => {
                let config = Self::extract_provider_config(args.env_file)?;
                CatalogApplication::run(&config).await?
            }
            CatalogCliCommands::Setup(args) => {
                let config = Self::extract_provider_config(args.env_file)?;
                CatalogMigration::run(&config).await?
            }
        }

        Ok(())
    }
}
