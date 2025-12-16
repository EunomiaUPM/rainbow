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

use super::app::Application;
use crate::ssi::consumer::setup::migrations::ConsumerMigration;
use crate::ssi::provider::setup::migrations::ProviderMigrations;
use clap::{Parser, Subcommand};
use rainbow_common::config::services::SsiAuthConfig;
use rainbow_common::config::traits::ConfigLoader;
use rainbow_common::config::types::roles::RoleConfig;
use std::cmp::PartialEq;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Aut Server")]
#[command(version = "0.1")]
struct AuthCli {
    #[command(subcommand)]
    role: AuthCliRoles,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum AuthCliRoles {
    #[command(subcommand)]
    Provider(AuthCliCommands),
    #[command(subcommand)]
    Consumer(AuthCliCommands),
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum AuthCliCommands {
    Start(AuthCliArgs),
    Setup(AuthCliArgs),
}

#[derive(Parser, Debug, PartialEq)]
pub struct AuthCliArgs {
    #[arg(short, long)]
    env_file: Option<String>,
}

pub struct AuthCommands;

impl AuthCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = AuthCli::parse();

        // run scripts
        match cli.role {
            AuthCliRoles::Provider(cmd) => match cmd {
                AuthCliCommands::Start(args) => {
                    let config = SsiAuthConfig::load(RoleConfig::Provider, args.env_file);
                    let table = json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                    info!("Current Auth Provider Config:\n{}", table);
                    Application::run(RoleConfig::Provider, config).await?;
                }
                AuthCliCommands::Setup(args) => {
                    let config = SsiAuthConfig::load(RoleConfig::Provider, args.env_file);
                    let table = json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                    info!("Current Auth Provider Config:\n{}", table);
                    ProviderMigrations::run(config).await?;
                }
            },
            AuthCliRoles::Consumer(cmd) => match cmd {
                AuthCliCommands::Start(args) => {
                    let config = SsiAuthConfig::load(RoleConfig::Consumer, args.env_file);
                    let table = json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                    info!("Current Auth Consumer Config:\n{}", table);
                    Application::run(RoleConfig::Consumer, config).await?;
                }
                AuthCliCommands::Setup(args) => {
                    let config = SsiAuthConfig::load(RoleConfig::Consumer, args.env_file);
                    let table = json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                    info!("Current Auth Consumer Config:\n{}", table);
                    ConsumerMigration::run(config).await?;
                }
            },
        };

        Ok(())
    }
}
