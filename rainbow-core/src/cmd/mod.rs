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

use crate::consumer::setup::application::CoreConsumerApplication;
use crate::consumer::setup::config::CoreApplicationConsumerConfig;
use crate::consumer::setup::db_migrations::CoreConsumerMigration;
use crate::provider::setup::application::CoreProviderApplication;
use crate::provider::setup::config::CoreApplicationProviderConfig;
use crate::provider::setup::db_migrations::CoreProviderMigration;
use clap::{Parser, Subcommand};
use rainbow_common::config::consumer_config::ApplicationConsumerConfigTrait;
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use std::cmp::PartialEq;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Core Server")]
#[command(version = "0.2")]
struct CoreCli {
    #[command(subcommand)]
    role: CoreCliRoles,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum CoreCliRoles {
    #[command(subcommand)]
    Provider(CoreCliCommands),
    #[command(subcommand)]
    Consumer(CoreCliCommands),
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum CoreCliCommands {
    Start,
    Setup,
}

pub struct CoreCommands;

impl CoreCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = CoreCli::parse();

        // run scripts
        match cli.role {
            CoreCliRoles::Provider(cmd) => {
                let config = CoreApplicationProviderConfig::default();
                let config = config.merge_dotenv_configuration();
                let mut config_table = config.clone();
                config_table.datahub_token = format!("{}...", config_table.datahub_token[0..20].to_string());
                let table =
                    json_to_table::json_to_table(&serde_json::to_value(&config_table)?).collapse().to_string();
                info!("Current config:\n{}", table);
                match cmd {
                    CoreCliCommands::Start => CoreProviderApplication::run(&config).await?,
                    CoreCliCommands::Setup => CoreProviderMigration::run(&config).await?,
                }
            }
            CoreCliRoles::Consumer(cmd) => {
                let config = CoreApplicationConsumerConfig::default();
                let config = config.merge_dotenv_configuration();
                let table =
                    json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                info!("Current config:\n{}", table);
                match cmd {
                    CoreCliCommands::Start => CoreConsumerApplication::run(&config).await?,
                    CoreCliCommands::Setup => CoreConsumerMigration::run(&config).await?,
                }
            }
        };

        Ok(())
    }
}
