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

use crate::ssi_auth::consumer::setup::app::SSIAuthConsumerApplication;
use crate::ssi_auth::consumer::setup::db_migrations::SSIAuthConsumerMigrations;
use crate::ssi_auth::provider::setup::app::SSIAuthProviderApplication;
use crate::ssi_auth::provider::setup::db_migrations::SSIAuthProviderMigrations;
use clap::{Parser, Subcommand};
use rainbow_common::config::env_extraction::EnvExtraction;
use std::cmp::PartialEq;
use tracing::debug;

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Auth Provider Server")]
#[command(version = "0.2")]
pub struct AuthCli {
    #[command(subcommand)]
    pub role: AuthCliRoles,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum AuthCliRoles {
    #[command(subcommand)]
    Provider(AuthCliCommands),
    #[command(subcommand)]
    Consumer(AuthCliCommands),
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum AuthCliCommands {
    Start(AuthCliArgs),
    Setup(AuthCliArgs),
}

#[derive(Parser, Debug, PartialEq)]
pub struct AuthCliArgs {
    #[arg(short, long)]
    pub env_file: Option<String>,
}

pub struct AuthCommands;

impl EnvExtraction for AuthCommands {}

impl AuthCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = AuthCli::parse();

        // run scripts
        match cli.role {
            AuthCliRoles::Provider(cmd) => match cmd {
                AuthCliCommands::Start(args) => {
                    let config = Self::extract_provider_config(args.env_file)?;
                    SSIAuthProviderApplication::run(&config).await?
                }
                AuthCliCommands::Setup(args) => {
                    let config = Self::extract_provider_config(args.env_file)?;
                    SSIAuthProviderMigrations::run(&config).await?
                }
            },
            AuthCliRoles::Consumer(cmd) => match cmd {
                AuthCliCommands::Start(args) => {
                    let config = Self::extract_consumer_config(args.env_file)?;
                    SSIAuthConsumerApplication::run(&config).await?
                }
                AuthCliCommands::Setup(args) => {
                    let config = Self::extract_consumer_config(args.env_file)?;
                    SSIAuthConsumerMigrations::run(&config).await?
                }
            },
        };

        Ok(())
    }
}
