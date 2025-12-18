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

use crate::gateway::consumer_gateway::RainbowConsumerGateway;
use crate::gateway::provider_gateway::RainbowProviderGateway;
use crate::subscriptions::consumer_subscriptions::RainbowConsumerGatewaySubscriptions;
use crate::subscriptions::provider_subscriptions::RainbowProviderGatewaySubscriptions;
use crate::subscriptions::MicroserviceSubscriptionKey;
use axum::serve;
use clap::{Parser, Subcommand};
use fs_extra::dir::{copy, CopyOptions};
use rainbow_common::config::services::GatewayConfig;
use rainbow_common::config::traits::{ConfigLoader, HostConfigTrait, IsLocalTrait};
use rainbow_common::config::types::roles::RoleConfig;
use rainbow_common::config::types::HostType;
use std::cmp::PartialEq;
use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::Context;
use tokio::net::TcpListener;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Gateway Server")]
#[command(version = "0.2")]
struct GatewayCli {
    #[command(subcommand)]
    role: GatewayCliRoles,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum GatewayCliRoles {
    #[command(subcommand)]
    Provider(GatewayCliCommands),
    #[command(subcommand)]
    Consumer(GatewayCliCommands),
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum GatewayCliCommands {
    Start(GatewayCliArgs),
    Subscribe(GatewayCliArgs),
    Build(GatewayCliArgs),
}

#[derive(Parser, Debug, PartialEq)]
pub struct GatewayCliArgs {
    #[arg(short, long)]
    env_file: Option<String>,
}

pub struct GatewayCommands;

impl GatewayCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = GatewayCli::parse();

        // run scripts
        match cli.role {
            GatewayCliRoles::Provider(cmd) => {
                match cmd {
                    GatewayCliCommands::Start(args) => {
                        let config = GatewayConfig::load(RoleConfig::Provider, args.env_file);
                        let gateway_router = RainbowProviderGateway::new(config.clone()).router();
                        let server_message = format!(
                            "Starting provider gateway server in {}",
                            config.get_host(HostType::Http)
                        );
                        info!("{}", server_message);
                        let listener = match config.is_local() {
                            true => TcpListener::bind(format!("127.0.0.1{}", config.get_weird_port())).await?,
                            false => TcpListener::bind(format!("0.0.0.0{}", config.get_weird_port())).await?,
                        };
                        serve(listener, gateway_router).await?;
                    }
                    GatewayCliCommands::Subscribe(args) => {
                        let config = GatewayConfig::load(RoleConfig::Provider, args.env_file);
                        let microservices_subs = RainbowProviderGatewaySubscriptions::new(config.clone());
                        microservices_subs.subscribe_to_microservice(MicroserviceSubscriptionKey::Catalog).await?;
                        // TODO when pubsub refactor
                        // microservices_subs.subscribe_to_microservice(MicroserviceSubscriptionKey::ContractNegotiation).await?;
                        // microservices_subs.subscribe_to_microservice(MicroserviceSubscriptionKey::TransferControlPlane).await?;
                    }
                    GatewayCliCommands::Build(args) => {
                        Self::build_frontend(RoleConfig::Provider, args.env_file)?;
                    }
                }
            }
            GatewayCliRoles::Consumer(cmd) => match cmd {
                GatewayCliCommands::Start(args) => {
                    let config = GatewayConfig::load(RoleConfig::Consumer, args.env_file);
                    let gateway_router = RainbowConsumerGateway::new(config.clone()).router();
                    let server_message = format!(
                        "Starting consumer gateway server in {}",
                        config.get_host(HostType::Http)
                    );
                    info!("{}", server_message);
                    let listener = match config.is_local() {
                        true => TcpListener::bind(format!("127.0.0.1{}", config.get_weird_port())).await?,
                        false => TcpListener::bind(format!("0.0.0.0{}", config.get_weird_port())).await?,
                    };
                    serve(listener, gateway_router).await?;
                }
                GatewayCliCommands::Subscribe(args) => {
                    let config = GatewayConfig::load(RoleConfig::Consumer, args.env_file);
                    let microservices_subs = RainbowConsumerGatewaySubscriptions::new(config.clone());
                    microservices_subs
                        .subscribe_to_microservice(MicroserviceSubscriptionKey::ContractNegotiation)
                        .await?;
                }
                GatewayCliCommands::Build(args) => {
                    Self::build_frontend(RoleConfig::Consumer, args.env_file)?;
                }
            },
        };

        Ok(())
    }

    fn build_frontend(role: RoleConfig, env_file: Option<String>) -> anyhow::Result<()> {
        let role = role.to_string().to_lowercase();
        let cwd = format!("./../gui/{}", role);

        // 1. Build react application
        let mut cmd = Command::new("npm")
            .current_dir(&cwd)
            .args(["run", "build", "-w", role.as_str()])
            .spawn()
            .context("Failed to spawn npm build process")?;

        cmd.wait().context("Failed to wait for npm build")?;
        debug!("Build command finished successfully");

        // 2. Rutas
        let origin = format!("{}/dist", cwd);
        let destination = format!("./src/static/{}", role);
        let dest_path = Path::new(&destination);

        // 3. Clean
        if dest_path.exists() {
            debug!("Cleaning content of: {}", destination);
            for entry in fs::read_dir(dest_path).context("Failed to read destination dir")? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(&path).context("Failed to remove subdir")?;
                } else {
                    fs::remove_file(&path).context("Failed to remove file")?;
                }
            }
        } else {
            fs::create_dir_all(dest_path).context("Failed to create destination dir")?;
        }

        // 4. Copy content
        let mut options = CopyOptions::new();
        options.overwrite = true;
        options.copy_inside = true;
        let _ = copy(&origin, &destination, &options)
            .context("Failed to execute copy process")?;

        debug!("Copy command finished successfully");

        Ok(())
    }
}
