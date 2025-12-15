/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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

use crate::setup::application::CatalogApplication;
use crate::setup::db_migrations::CatalogAgentMigration;
use clap::{Parser, Subcommand};
use rainbow_common::config::env_extraction::EnvExtraction;
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use tracing::debug;

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Catalog Agent")]
#[command(version = "0.2")]
struct CatalogCli {
    #[clap(subcommand)]
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

pub struct CatalogCommands {}
impl EnvExtraction for CatalogCommands {}

impl CatalogCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        debug!("init_command_line - Initialize catalog commands");
        let cli = CatalogCli::parse();
        match cli.command {
            CatalogCliCommands::Start(args) => {
                let config = Self::extract_provider_config(args.env_file)?;
                let global_config: ApplicationGlobalConfig = config.into();
                CatalogApplication::run(&global_config).await?;
            }
            CatalogCliCommands::Setup(args) => {
                let config = Self::extract_provider_config(args.env_file)?;
                let global_config: ApplicationGlobalConfig = config.into();
                CatalogAgentMigration::run(&global_config).await?;
            }
        }
        Ok(())
    }
}
