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

use crate::consumer::db_migrations::CoreConsumerMigration;
use crate::provider::db_migrations::CoreProviderMigration;
use crate::provider::db_seeding::CoreProviderSeeding;
use crate::setup::CoreApplication;
use clap::{Parser, Subcommand};
use rainbow_common::config::services::traits::MonoConfigTrait;
use rainbow_common::config::traits::ConfigLoader;
use rainbow_common::config::types::roles::RoleConfig;
use rainbow_common::config::ApplicationConfig;
use std::cmp::PartialEq;
use tracing::debug;

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
    Start(CoreCliArgs),
    Setup(CoreCliArgs),
}

#[derive(Parser, Debug, PartialEq)]
pub struct CoreCliArgs {
    #[arg(short, long)]
    env_file: Option<String>,
}

pub struct CoreCommands;

impl CoreCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = CoreCli::parse();

        // run scripts
        match cli.role {
            CoreCliRoles::Provider(cmd) => match cmd {
                CoreCliCommands::Start(args) => {
                    let config = ApplicationConfig::load(RoleConfig::Provider, args.env_file);
                    CoreApplication::run(RoleConfig::Provider, &config).await?
                }
                CoreCliCommands::Setup(args) => {
                    let config = ApplicationConfig::load(RoleConfig::Provider, args.env_file);
                    CoreProviderMigration::run(&config).await?;
                    match config.is_mono_catalog_datahub() {
                        true => {}
                        false => {
                            CoreProviderSeeding::run(&config).await?;
                        }
                    }
                }
            },
            CoreCliRoles::Consumer(cmd) => match cmd {
                CoreCliCommands::Start(args) => {
                    let config = ApplicationConfig::load(RoleConfig::Consumer, args.env_file);
                    CoreApplication::run(RoleConfig::Consumer, &config).await?
                }
                CoreCliCommands::Setup(args) => {
                    let config = ApplicationConfig::load(RoleConfig::Consumer, args.env_file);
                    CoreConsumerMigration::run(&config).await?
                }
            },
        };

        Ok(())
    }
}
