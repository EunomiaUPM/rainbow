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

use crate::setup::databases::setup_database;
use crate::setup::servers::{start_consumer_server, start_provider_server};
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use dotenvy::dotenv;
use rainbow_common::config::config::{Config, ConfigRoles, GLOBAL_CONFIG};
use rainbow_common::config_field;
use std::env;
use tracing::{debug, error, info};

#[derive(Parser, Debug)]
#[command(name = "Dataspace protocol")]
#[command(version = "0.1")]
#[command(about = "Dataspace protocol", long_about = "Dataspace protocol")]
struct Cli {
    #[command(subcommand)]
    role: DataSpaceTransferRoles,
}

#[derive(Subcommand, Debug)]
pub enum DataSpaceTransferRoles {
    #[command(about = "Start the consumer testing scripts")]
    Consumer(ConsumerArgs),
    #[command(about = "Start the provider server")]
    Provider(ProviderArgs),
}

#[derive(Debug, Args)]
pub struct ConsumerArgs {
    #[arg(long)]
    provider_url: Option<String>,
    #[arg(long)]
    provider_port: Option<String>,
    #[arg(long)]
    host_url: Option<String>,
    #[arg(long)]
    host_port: Option<String>,
    #[arg(long)]
    db_type: Option<String>,
    #[arg(long)]
    db_url: Option<String>,
    #[arg(long)]
    db_port: Option<String>,
    #[arg(long)]
    db_user: Option<String>,
    #[arg(long)]
    db_password: Option<String>,
    #[arg(long)]
    db_database: Option<String>,
    #[command(subcommand)]
    command: ConsumerCommands,
}

#[derive(Debug, Args)]
pub struct ProviderArgs {
    #[arg(long)]
    host_url: Option<String>,
    #[arg(long)]
    host_port: Option<String>,
    #[arg(long)]
    db_type: Option<String>,
    #[arg(long)]
    db_url: Option<String>,
    #[arg(long)]
    db_port: Option<String>,
    #[arg(long)]
    db_user: Option<String>,
    #[arg(long)]
    db_password: Option<String>,
    #[arg(long)]
    db_database: Option<String>,
    #[arg(long)]
    auth_url: Option<String>,
    #[arg(long)]
    auth_port: Option<String>,
    #[command(subcommand)]
    command: ProviderCommands,
}

#[derive(Subcommand, Debug)]
pub enum ProviderCommands {
    Start {},
    Setup,
    LoadContracts,
}

#[derive(Subcommand, Debug)]
pub enum ConsumerCommands {
    Start {},
    Setup,
}

pub async fn init_command_line() -> Result<()> {
    info!("Init the command line application");
    let cli = Cli::parse();
    if env::var_os("TEST").is_none() {
        dotenv().ok();
    }

    let config = match &cli.role {
        DataSpaceTransferRoles::Provider(args) => Config {
            host_url: config_field!(args, host_url, "HOST_URL", "localhost"),
            host_port: config_field!(args, host_port, "HOST_PORT", "1234"),
            db_type: config_field!(args, db_type, "DB_TYPE", "postgres"),
            db_url: config_field!(args, db_url, "DB_URL", "localhost"),
            db_port: config_field!(args, db_port, "DB_PORT", "5433"),
            db_user: config_field!(args, db_user, "DB_USER", "ds-protocol-provider"),
            db_password: config_field!(args, db_password, "DB_PASSWORD", "ds-protocol-provider"),
            db_database: config_field!(args, db_database, "DB_DATABASE", "ds-protocol-provider"),
            provider_url: None,
            provider_port: None,
            auth_url: Some(config_field!(args, auth_url, "AUTH_URL", "localhost")),
            auth_port: Some(config_field!(args, auth_port, "AUTH_PORT", "1232")),
            role: ConfigRoles::Provider,
            ssi_auth_enabled: Some(config_field!(
                args,
                db_database,
                "DB_DATABASE",
                "ds-protocol-provider"
            )),
            ssi_holder_url: Some(config_field!(
                args,
                db_database,
                "DB_DATABASE",
                "ds-protocol-provider"
            )),
            ssi_holder_port: Some(config_field!(
                args,
                db_database,
                "DB_DATABASE",
                "ds-protocol-provider"
            )),
            ssi_verifier_url: Some(config_field!(
                args,
                db_database,
                "SSI_VERIFIER_URL",
                "ds-protocol-provider"
            )),
            ssi_verifier_port: Some(config_field!(
                args,
                db_database,
                "SSI_VERIFIER_PORT",
                "ds-protocol-provider"
            )),
            catalog_as_service: None,
            catalog_service_url: None,
            catalog_service_port: None,
            contracts_as_service: None,
            contracts_service_url: None,
            contracts_service_port: None,
        },
        DataSpaceTransferRoles::Consumer(args) => Config {
            host_url: config_field!(args, host_url, "HOST_URL", "localhost"),
            host_port: config_field!(args, host_port, "HOST_PORT", "1235"),
            db_type: config_field!(args, db_type, "DB_TYPE", "postgres"),
            db_url: config_field!(args, db_url, "DB_URL", "localhost"),
            db_port: config_field!(args, db_port, "DB_PORT", "5434"),
            db_user: config_field!(args, db_user, "DB_USER", "ds-protocol-consumer"),
            db_password: config_field!(args, db_password, "DB_PASSWORD", "ds-protocol-consumer"),
            db_database: config_field!(args, db_database, "DB_DATABASE", "ds-protocol-consumer"),
            provider_url: Some(config_field!(
                args,
                provider_url,
                "PROVIDER_HOST",
                "localhost"
            )),
            provider_port: Some(config_field!(args, provider_port, "PROVIDER_PORT", "1234")),
            auth_url: None,
            auth_port: None,
            role: ConfigRoles::Consumer,
            ssi_auth_enabled: Some(config_field!(
                args,
                db_database,
                "DB_DATABASE",
                "ds-protocol-provider"
            )),
            ssi_holder_url: Some(config_field!(
                args,
                db_database,
                "DB_DATABASE",
                "ds-protocol-provider"
            )),
            ssi_holder_port: Some(config_field!(
                args,
                db_database,
                "DB_DATABASE",
                "ds-protocol-provider"
            )),
            ssi_verifier_url: Some(config_field!(
                args,
                db_database,
                "SSI_VERIFIER_URL",
                "ds-protocol-provider"
            )),
            ssi_verifier_port: Some(config_field!(
                args,
                db_database,
                "SSI_VERIFIER_PORT",
                "ds-protocol-provider"
            )),
            catalog_as_service: None,
            catalog_service_url: None,
            catalog_service_port: None,
            contracts_as_service: None,
            contracts_service_url: None,
            contracts_service_port: None,
        },
    };

    GLOBAL_CONFIG.set(config).expect("Global Config not initialized");

    info!(
        "Config status: \n{}",
        serde_json::to_string_pretty(GLOBAL_CONFIG.get().unwrap())?
    );

    match &cli.role {
        DataSpaceTransferRoles::Consumer(args) => {
            // CONFIG FOR CONSUMER HERE
            match &args.command {
                ConsumerCommands::Start { .. } => {
                    start_consumer_server().await?;
                }
                ConsumerCommands::Setup => {
                    setup_database("consumer".to_string()).await?;
                }
            }
        }
        DataSpaceTransferRoles::Provider(args) => {
            // CONFIG FOR PROVIDER HERE
            match &args.command {
                ProviderCommands::Start { .. } => {
                    start_provider_server().await?;
                }
                ProviderCommands::Setup => {
                    setup_database("provider".to_string()).await?;
                }
                ProviderCommands::LoadContracts => {}
            }
        }
    }

    Ok(())
}
