/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::cmp::PartialEq;

use clap::{Parser, Subcommand};
use rainbow_common::config::services::SsiAuthConfig;
use rainbow_common::config::traits::ConfigLoader;
use rainbow_common::vault::vault_rs::VaultService;
use rainbow_common::vault::VaultTrait;
use tracing::{debug, info};

use super::app::Application;
use crate::ssi::setup::migrations::AuthMigrator;

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Aut Server")]
#[command(version = "0.1")]
struct AuthCli {
    #[command(subcommand)]
    command: AuthCliCommands
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum AuthCliCommands {
    Start(AuthCliArgs),
    Setup(AuthCliArgs)
}

#[derive(Parser, Debug, PartialEq)]
pub struct AuthCliArgs {
    #[arg(short, long)]
    env_file: Option<String>
}

pub struct AuthCommands;

impl AuthCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = AuthCli::parse();
        let vault = VaultService::new();

        match cli.command {
            AuthCliCommands::Start(args) => {
                let config = SsiAuthConfig::load(args.env_file);
                let table = json_to_table::json_to_table(&serde_json::to_value(&config)?)
                    .collapse()
                    .to_string();
                info!("Current Auth Config:\n{}", table);
                Application::run(config, vault).await?;
            }
            AuthCliCommands::Setup(args) => {
                let config = SsiAuthConfig::load(args.env_file);
                let table = json_to_table::json_to_table(&serde_json::to_value(&config)?)
                    .collapse()
                    .to_string();
                info!("Current Auth Config:\n{}", table);
                let connection = vault.get_connection(config).await;
                AuthMigrator::run(connection).await?;
            }
        }

        Ok(())
    }
}
