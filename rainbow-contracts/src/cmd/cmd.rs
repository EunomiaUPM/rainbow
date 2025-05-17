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

use crate::consumer::setup::application::ContractNegotiationConsumerApplication;
use crate::consumer::setup::config::ContractNegotiationConsumerApplicationConfig;
use crate::consumer::setup::db_migrations::ContractNegotiationConsumerMigration;
use crate::provider::setup::application::ContractNegotiationProviderApplication;
use crate::provider::setup::config::ContractNegotiationApplicationProviderConfig;
use crate::provider::setup::db_migrations::ContractNegotiationProviderMigration;
use clap::{Parser, Subcommand};
use rainbow_common::config::consumer_config::ApplicationConsumerConfigTrait;
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use tracing::{debug, info};

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
    Start,
    Setup,
}

pub struct ContractNegotiationCommands;

impl ContractNegotiationCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = ContractNegotiationCli::parse();

        // run scripts
        match cli.role {
            ContractNegotiationCliRoles::Provider(cmd) => {
                let config = ContractNegotiationApplicationProviderConfig::default();
                let config = config.merge_dotenv_configuration();
                let table =
                    json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                info!("Current config:\n{}", table);
                match cmd {
                    ContractNegotiationCliCommands::Start => ContractNegotiationProviderApplication::run(&config).await?,
                    ContractNegotiationCliCommands::Setup => ContractNegotiationProviderMigration::run(&config).await?,
                }
            }
            ContractNegotiationCliRoles::Consumer(cmd) => {
                let config = ContractNegotiationConsumerApplicationConfig::default();
                let config = config.merge_dotenv_configuration();
                let table =
                    json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                info!("Current config:\n{}", table);
                match cmd {
                    ContractNegotiationCliCommands::Start => ContractNegotiationConsumerApplication::run(&config.clone()).await?,
                    ContractNegotiationCliCommands::Setup => ContractNegotiationConsumerMigration::run(&config.clone()).await?
                }
            }
        };

        Ok(())
    }
}