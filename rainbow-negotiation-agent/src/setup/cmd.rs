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

use crate::setup::application::NegotiationAgentApplication;
use crate::setup::db_migrations::NegotiationAgentMigration;
use clap::{Parser, Subcommand};
use rainbow_common::config::env_extraction::EnvExtraction;
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use tracing::debug;

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Negotiation Agent")]
#[command(version = "0.2")]
struct NegotiationCli {
    #[clap(subcommand)]
    command: NegotiationCliCommands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum NegotiationCliCommands {
    Start(NegotiationCliArgs),
    Setup(NegotiationCliArgs),
}

#[derive(Parser, Debug, PartialEq)]
pub struct NegotiationCliArgs {
    #[arg(short, long)]
    env_file: Option<String>,
}

pub struct NegotiationCommands {}
impl EnvExtraction for NegotiationCommands {}

impl NegotiationCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        debug!("init_command_line - Initialize negotiation commands");
        let cli = NegotiationCli::parse();
        match cli.command {
            NegotiationCliCommands::Start(args) => {
                let config = Self::extract_provider_config(args.env_file)?;
                let config_as_global: ApplicationGlobalConfig = config.into();
                NegotiationAgentApplication::run(&config_as_global).await?;
            }
            NegotiationCliCommands::Setup(args) => {
                let config = Self::extract_provider_config(args.env_file)?;
                let config_as_global: ApplicationGlobalConfig = config.into();
                NegotiationAgentMigration::run(&config_as_global).await?;
            }
        }
        Ok(())
    }
}
