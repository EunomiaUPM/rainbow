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

use crate::setup::application::DatahubCatalogApplication;
use crate::setup::config::DatahubCatalogApplicationProviderConfig;
use crate::setup::db_migrations::DatahubCatalogRelationsMigration;
use clap::{Parser, Subcommand};
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use std::cmp::PartialEq;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Datahub Catalog Server")]
#[command(version = "0.2")]
struct CatalogCli {
    #[command(subcommand)]
    command: CatalogCliCommands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum CatalogCliCommands {
    Start,
    Setup,
}

pub struct CatalogCommands;

impl CatalogCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = CatalogCli::parse();

        // config
        let config = DatahubCatalogApplicationProviderConfig::default();
        let config = config.merge_dotenv_configuration();
        let mut config_table = config.clone();
        config_table.datahub_token = format!("{}...", config_table.datahub_token[0..20].to_string());
        let table =
            json_to_table::json_to_table(&serde_json::to_value(&config_table)?).collapse().to_string();
        info!("Current config:\n{}", table);

        // run scripts
        match cli.command {
            CatalogCliCommands::Start => DatahubCatalogApplication::run(&config).await?,
            CatalogCliCommands::Setup => DatahubCatalogRelationsMigration::run(&config).await?,
        }

        Ok(())
    }
}
