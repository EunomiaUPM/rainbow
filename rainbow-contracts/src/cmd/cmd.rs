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

use crate::consumer::setup::application::ContractNegotiationConsumerApplication;
use crate::consumer::setup::db_migrations::ContractNegotiationConsumerMigration;
use crate::provider::setup::application::ContractNegotiationProviderApplication;
use crate::provider::setup::db_migrations::ContractNegotiationProviderMigration;
use clap::{Parser, Subcommand};
use rainbow_common::config::services::ContractsConfig;
use rainbow_common::config::traits::ConfigLoader;
use rainbow_common::config::types::roles::RoleConfig;
use tracing::debug;

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Contract Negotiation Server")]
#[command(version = "0.2")]
struct ContractNegotiationCli {
    #[command(subcommand)]
    role: ContractNegotiationCliRoles,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum ContractNegotiationCliRoles {
    #[command(subcommand)]
    Provider(ContractNegotiationCliCommands),
    #[command(subcommand)]
    Consumer(ContractNegotiationCliCommands),
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum ContractNegotiationCliCommands {
    Start(ContractNegotiationCliArgs),
    Setup(ContractNegotiationCliArgs),
}

#[derive(Parser, Debug, PartialEq)]
pub struct ContractNegotiationCliArgs {
    #[arg(short, long)]
    env_file: Option<String>,
}

pub struct ContractNegotiationCommands;

impl ContractNegotiationCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = ContractNegotiationCli::parse();

        // run scripts
        match cli.role {
            ContractNegotiationCliRoles::Provider(cmd) => match cmd {
                ContractNegotiationCliCommands::Start(args) => {
                    let config = ContractsConfig::load(RoleConfig::Provider, args.env_file);
                    ContractNegotiationProviderApplication::run(&config).await?
                }
                ContractNegotiationCliCommands::Setup(args) => {
                    let config = ContractsConfig::load(RoleConfig::Provider, args.env_file);
                    ContractNegotiationProviderMigration::run(&config).await?
                }
            },
            ContractNegotiationCliRoles::Consumer(cmd) => match cmd {
                ContractNegotiationCliCommands::Start(args) => {
                    let config = ContractsConfig::load(RoleConfig::Consumer, args.env_file);
                    ContractNegotiationConsumerApplication::run(&config).await?
                }
                ContractNegotiationCliCommands::Setup(args) => {
                    let config = ContractsConfig::load(RoleConfig::Consumer, args.env_file);
                    ContractNegotiationConsumerMigration::run(&config).await?
                }
            },
        };

        Ok(())
    }
}
