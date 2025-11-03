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

use crate::provider::setup::application::CatalogApplication;
use crate::provider::setup::db_migrations::CatalogMigration;
use clap::{Parser, Subcommand};
use rainbow_common::config::consumer_config::{ApplicationConsumerConfig, ApplicationConsumerConfigTrait};
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use std::cmp::PartialEq;
use tracing::{debug, info};
use crate::provider::setup::db_seeding::CatalogSeeding;

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Catalog Server")]
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
                CatalogMigration::run(&config).await?;
                CatalogSeeding::run(&config).await?
            }
        }

        Ok(())
    }
    fn extract_provider_config(env_file: Option<String>) -> anyhow::Result<ApplicationProviderConfig> {
        let config = ApplicationProviderConfig::default();
        let config = config.merge_dotenv_configuration(env_file);
        let mut config_table = config.clone();
        config_table.datahub_token = format!("{}...", config_table.datahub_token[0..20].to_string());
        let table =
            json_to_table::json_to_table(&serde_json::to_value(&config_table)?).collapse().to_string();
        info!("Current Application Provider Config:\n{}", table);
        Ok(config)
    }
    fn extract_consumer_config(env_file: Option<String>) -> anyhow::Result<ApplicationConsumerConfig> {
        let config = ApplicationConsumerConfig::default();
        let config = config.merge_dotenv_configuration(env_file);
        let table =
            json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
        info!("Current Application Consumer Config:\n{}", table);
        Ok(config)
    }
}
