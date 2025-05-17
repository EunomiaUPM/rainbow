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

use crate::consumer::setup::application::TransferConsumerApplication;
use crate::consumer::setup::config::TransferConsumerApplicationConfig;
use crate::consumer::setup::db_migrations::TransferConsumerMigration;
use crate::provider::setup::application::TransferProviderApplication;
use crate::provider::setup::config::TransferProviderApplicationConfig;
use crate::provider::setup::db_migrations::TransferProviderMigration;
use clap::{Parser, Subcommand};
use rainbow_common::config::consumer_config::ApplicationConsumerConfigTrait;
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use std::cmp::PartialEq;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Transfer Provider Server")]
#[command(version = "0.2")]
struct TransferCli {
    #[command(subcommand)]
    role: TransferCliRoles,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum TransferCliRoles {
    #[command(subcommand)]
    Provider(TransferCliCommands),
    #[command(subcommand)]
    Consumer(TransferCliCommands),
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum TransferCliCommands {
    Start,
    Setup,
}

pub struct TransferCommands;

impl TransferCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = TransferCli::parse();

        // run scripts
        match cli.role {
            TransferCliRoles::Provider(cmd) => {
                let config = TransferProviderApplicationConfig::default();
                let config = config.merge_dotenv_configuration();
                let table =
                    json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                info!("Current config:\n{}", table);
                match cmd {
                    TransferCliCommands::Start => TransferProviderApplication::run(&config).await?,
                    TransferCliCommands::Setup => TransferProviderMigration::run(&config).await?,
                }
            }
            TransferCliRoles::Consumer(cmd) => {
                let config = TransferConsumerApplicationConfig::default();
                let config = config.merge_dotenv_configuration();
                let table =
                    json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                info!("Current config:\n{}", table);
                match cmd {
                    TransferCliCommands::Start => TransferConsumerApplication::run(&config.clone()).await?,
                    TransferCliCommands::Setup => TransferConsumerMigration::run(config.clone()).await?
                }
            }
        };

        Ok(())
    }
}
