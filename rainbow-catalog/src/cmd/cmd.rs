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

use crate::consumer::setup::application::CatalogBypassConsumerApplication;
use crate::consumer::setup::db_migrations::CatalogBypassConsumerMigration;
use crate::provider::setup::application::CatalogApplication;
use crate::provider::setup::db_migrations::CatalogMigration;
use clap::{Parser, Subcommand};
use rainbow_common::config::consumer_config::{ApplicationConsumerConfig, ApplicationConsumerConfigTrait};
use rainbow_common::config::env_extraction::EnvExtraction;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use std::cmp::PartialEq;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Catalog Server")]
#[command(version = "0.2")]
struct CatalogCli {
    #[command(subcommand)]
    role: CatalogCliRoles,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum CatalogCliRoles {
    #[command(subcommand)]
    Provider(CatalogCliCommands),
    #[command(subcommand)]
    Consumer(CatalogCliCommands),
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

impl EnvExtraction for CatalogCommands {}

impl CatalogCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = CatalogCli::parse();

        // run scripts
        match cli.role {
            CatalogCliRoles::Provider(cmd) => match cmd {
                CatalogCliCommands::Start(args) => {
                    let config = Self::extract_provider_config(args.env_file)?;
                    CatalogApplication::run(&config).await?
                }
                CatalogCliCommands::Setup(args) => {
                    let config = Self::extract_provider_config(args.env_file)?;
                    CatalogMigration::run(&config).await?
                }
            },
            CatalogCliRoles::Consumer(cmd) => match cmd {
                CatalogCliCommands::Start(args) => {
                    let config = Self::extract_consumer_config(args.env_file)?;
                    CatalogBypassConsumerApplication::run(&config).await?
                }
                CatalogCliCommands::Setup(args) => {
                    let config = Self::extract_consumer_config(args.env_file)?;
                    CatalogBypassConsumerMigration::run(&config).await?
                }
            },
        };

        Ok(())
    }
}
