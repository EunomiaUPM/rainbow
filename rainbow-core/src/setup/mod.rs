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

use crate::http::{get_consumer_routes, get_provider_routes};
use crate::migrations::{migrate_consumer_db, migrate_provider_db};
use anyhow::Result;
use axum::serve;
use clap::{Args, Parser, Subcommand};
use dotenvy::dotenv;
use rainbow_common::config::config::{Config, ConfigRoles, GLOBAL_CONFIG};
use rainbow_common::config_field;
use sea_orm_migration::MigratorTrait;
use std::env;
use tokio::net::TcpListener;
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
    #[arg(long)]
    ssi_auth_enabled: Option<String>,
    #[arg(long)]
    ssi_holder_url: Option<String>,
    #[arg(long)]
    ssi_holder_port: Option<String>,
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
    #[arg(long)]
    ssi_auth_enabled: Option<String>,
    #[arg(long)]
    ssi_holder_url: Option<String>,
    #[arg(long)]
    ssi_holder_port: Option<String>,
    #[arg(long)]
    ssi_verifier_url: Option<String>,
    #[arg(long)]
    ssi_verifier_port: Option<String>,
    #[arg(long)]
    catalog_as_service: Option<String>,
    #[arg(long)]
    catalog_service_url: Option<String>,
    #[arg(long)]
    catalog_service_port: Option<String>,
    #[arg(long)]
    contracts_as_service: Option<String>,
    #[arg(long)]
    contracts_service_url: Option<String>,
    #[arg(long)]
    contracts_service_port: Option<String>,
    #[command(subcommand)]
    command: ProviderCommands,
}

#[derive(Subcommand, Debug)]
pub enum ProviderCommands {
    Start {},
    Setup,
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
            host_url: config_field!(args, host_url, "HOST_URL", "127.0.0.1"),
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
            ssi_holder_wallet_portal_url: Some(String::from("http://localhost")), // COMPLETAR
            ssi_holder_wallet_portal_port: Some(String::from("7001")),
            ssi_holder_wallet_type: Some(String::from("email")),
            ssi_holder_wallet_name: Some(String::from("pepe")),
            ssi_holder_wallet_email: Some(String::from("kk@kk.com")),
            ssi_holder_wallet_password: Some(String::from("kk")),
            ssi_holder_wallet_id: None,

            role: ConfigRoles::Provider,
            ssi_auth_enabled: Some(config_field!(
                args,
                ssi_auth_enabled,
                "SSI_AUTH_ENABLED",
                "false"
            )),
            ssi_holder_url: None,
            ssi_holder_port: None,
            ssi_verifier_url: Some(config_field!(
                args,
                ssi_verifier_url,
                "SSI_VERIFIER_URL",
                "127.0.0.1"
            )),
            ssi_verifier_port: Some(config_field!(
                args,
                ssi_holder_port,
                "SSI_VERIFIER_PORT",
                "4001"
            )),
            catalog_as_service: Some(config_field!(
                args,
                catalog_as_service,
                "CATALOG_AS_SERVICE",
                "false"
            )),
            catalog_service_url: Some(config_field!(
                args,
                catalog_service_url,
                "CATALOG_SERVICE_URL",
                "127.0.0.1"
            )),
            catalog_service_port: Some(config_field!(
                args,
                catalog_service_port,
                "CATALOG_SERVICE_PORT",
                "1232"
            )),
            contracts_as_service: Some(config_field!(
                args,
                catalog_as_service,
                "CONTRACTS_AS_SERVICE",
                "false"
            )),
            contracts_service_url: Some(config_field!(
                args,
                contracts_service_url,
                "CONTRACTS_SERVICE_URL",
                "127.0.0.1"
            )),
            contracts_service_port: Some(config_field!(
                args,
                contracts_service_port,
                "CONTRACTS_SERVICE_PORT",
                "1232"
            )),
            provider_verification_portal_url: Some(String::from("http://host.docker.internal:1234")),
        },
        DataSpaceTransferRoles::Consumer(args) => Config {
            host_url: config_field!(args, host_url, "HOST_URL", "127.0.0.1"),
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
                "127.0.0.1"
            )),
            provider_port: Some(config_field!(args, provider_port, "PROVIDER_PORT", "1234")),
            auth_url: None,
            auth_port: None,
            ssi_holder_wallet_portal_url: Some(String::from("http://localhost")), // COMPLETAR
            ssi_holder_wallet_portal_port: Some(String::from("7001")),
            ssi_holder_wallet_type: Some(String::from("email")),
            ssi_holder_wallet_name: Some(String::from("pepe")),
            ssi_holder_wallet_email: Some(String::from("kk@kk.com")),
            ssi_holder_wallet_password: Some(String::from("kk")),
            ssi_holder_wallet_id: None,
            role: ConfigRoles::Consumer,
            ssi_auth_enabled: Some(config_field!(
                args,
                ssi_auth_enabled,
                "SSI_AUTH_ENABLED",
                "false"
            )),
            ssi_holder_url: Some(config_field!(
                args,
                ssi_holder_url,
                "SSI_HOLDER_URL",
                "127.0.0.1"
            )),
            ssi_holder_port: Some(config_field!(
                args,
                ssi_holder_port,
                "SSI_HOLDER_PORT",
                "4000"
            )),
            ssi_verifier_url: None,
            ssi_verifier_port: None,
            catalog_as_service: None,
            catalog_service_url: None,
            catalog_service_port: None,
            contracts_as_service: None,
            contracts_service_url: None,
            contracts_service_port: None,
            provider_verification_portal_url: None,
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
                    let config = GLOBAL_CONFIG.get().unwrap();
                    let server_message = format!(
                        "Starting consumer server in http://{}:{}",
                        config.host_url, config.host_port
                    );
                    info!("{}", server_message);
                    let listener =
                        TcpListener::bind(format!("{}:{}", config.host_url, config.host_port))
                            .await?;
                    serve(listener, get_consumer_routes().await).await?;
                }
                ConsumerCommands::Setup => {
                    migrate_consumer_db().await?;
                }
            }
        }
        DataSpaceTransferRoles::Provider(args) => {
            // CONFIG FOR PROVIDER HERE
            match &args.command {
                ProviderCommands::Start { .. } => {
                    let config = GLOBAL_CONFIG.get().unwrap();
                    let server_message = format!(
                        "Starting provider server in http://{}:{}",
                        config.host_url, config.host_port
                    );
                    info!("{}", server_message);
                    let listener =
                        TcpListener::bind(format!("{}:{}", config.host_url, config.host_port))
                            .await?;
                    serve(listener, get_provider_routes().await).await?;
                }
                ProviderCommands::Setup => {
                    migrate_provider_db().await?;
                }
            }
        }
    }

    Ok(())
}
