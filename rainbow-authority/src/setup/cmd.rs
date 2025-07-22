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

use crate::setup::application::AuthorityApplication;
use crate::setup::config::AuthorityApplicationConfig;
use crate::setup::db_migrations::AuthorityMigration;
use clap::{Parser, Subcommand};
use rainbow_common::config::consumer_config::ApplicationConsumerConfigTrait;
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use std::cmp::PartialEq;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Authority Server")]
#[command(version = "0.1")]
struct AuthorityCli {
    #[command(subcommand)]
    command: AuthorityCliCommands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum AuthorityCliCommands {
    Start,
    Setup,
}

pub struct AuthorityCommands;

impl AuthorityCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = AuthorityCli::parse();

        // run scripts
        let config = AuthorityApplicationConfig::default();
        let config = config.merge_dotenv_configuration();
        let table =
            json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
        info!("Current config:\n{}", table);
        match cli.command {
            AuthorityCliCommands::Start => AuthorityApplication::run(&config).await?,
            AuthorityCliCommands::Setup => AuthorityMigration::run(&config).await?,
        }

        Ok(())
    }
}
