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

use crate::setup::application::TransferApplication;
use crate::setup::db_migrations::TransferAgentMigration;
use clap::{Parser, Subcommand};
use rainbow_common::config::env_extraction::EnvExtraction;
use tracing::debug;

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Transfer Agent")]
#[command(version = "0.2")]
struct TransferCli {
    #[clap(subcommand)]
    command: TransferCliCommands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum TransferCliCommands {
    Start(TransferCliArgs),
    Setup(TransferCliArgs),
}

#[derive(Parser, Debug, PartialEq)]
pub struct TransferCliArgs {
    #[arg(short, long)]
    env_file: Option<String>,
}

pub struct TransferCommands {}
impl EnvExtraction for TransferCommands {}

impl TransferCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        debug!("init_command_line - Initialize transfer commands");
        let cli = TransferCli::parse();
        match cli.command {
            TransferCliCommands::Start(args) => {
                let config = Self::extract_provider_config(args.env_file)?;
                TransferApplication::run(&config).await?;
            }
            TransferCliCommands::Setup(args) => {
                let config = Self::extract_provider_config(args.env_file)?;
                TransferAgentMigration::run(&config).await?;
            }
        }
        Ok(())
    }
}
