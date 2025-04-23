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

use crate::setup::application::CatalogApplication;
use crate::setup::config::CatalogApplicationConfig;
use crate::setup::db_migrations::CatalogMigration;
use clap::{Parser, Subcommand};
use std::cmp::PartialEq;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Catalog Server")]
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
        let config = CatalogApplicationConfig::default();
        let config = config.merge_dotenv_configuration().unwrap_or_else(|_| config);

        let table =
            json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
        info!("Current config:\n{}", table);

        // run scripts
        match cli.command {
            CatalogCliCommands::Start => CatalogApplication::run(config.clone()).await?,
            CatalogCliCommands::Setup => CatalogMigration::run(config.clone()).await?,
        }

        Ok(())
    }
}
