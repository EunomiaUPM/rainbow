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

use crate::gateway::consumer_gateway::RainbowConsumerGateway;
use crate::gateway::provider_gateway::RainbowProviderGateway;
use crate::subscriptions::provider_subscriptions::RainbowProviderGatewaySubscriptions;
use crate::subscriptions::MicroserviceSubscriptionKey;
use axum::serve;
use clap::{Parser, Subcommand};
use rainbow_common::config::consumer_config::{ApplicationConsumerConfig, ApplicationConsumerConfigTrait};
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use std::cmp::PartialEq;
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
    Start,
    Subscribe,
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
                let config = ApplicationProviderConfig::default();
                let config = config.merge_dotenv_configuration();
                let table = json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                info!("Current config:\n{}", table);
                match cmd {
                    GatewayCliCommands::Start => {
                        let gateway_router = RainbowProviderGateway::new(config.clone()).router();
                        let server_message = format!(
                            "Starting provider gateway server in {}",
                            config.get_gateway_host_url().unwrap()
                        );
                        info!("{}", server_message);
                        let listener = TcpListener::bind(format!(
                            "{}:{}",
                            config.get_raw_gateway_host().clone().unwrap().url,
                            config.get_raw_gateway_host().clone().unwrap().port
                        ))
                            .await?;
                        serve(listener, gateway_router).await?;
                    }
                    GatewayCliCommands::Subscribe => {
                        let microservices_subs = RainbowProviderGatewaySubscriptions::new(config.clone());
                        microservices_subs.subscribe_to_microservice(MicroserviceSubscriptionKey::Catalog).await?;
                        // TODO when pubsub refactor
                        // microservices_subs.subscribe_to_microservice(MicroserviceSubscriptionKey::ContractNegotiation).await?;
                        // microservices_subs.subscribe_to_microservice(MicroserviceSubscriptionKey::TransferControlPlane).await?;
                    }
                }
            }
            GatewayCliRoles::Consumer(cmd) => {
                let config = ApplicationConsumerConfig::default();
                let config = config.merge_dotenv_configuration();
                let table = json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                info!("Current config:\n{}", table);
                match cmd {
                    GatewayCliCommands::Start => {
                        let gateway_router = RainbowConsumerGateway::new(config.clone()).router();
                        let server_message = format!(
                            "Starting consumer gateway server in {}",
                            config.get_gateway_host_url().unwrap()
                        );
                        info!("{}", server_message);
                        let listener = TcpListener::bind(format!(
                            "{}:{}",
                            config.get_raw_gateway_host().clone().unwrap().url,
                            config.get_raw_gateway_host().clone().unwrap().port
                        ))
                            .await?;
                        serve(listener, gateway_router).await?;
                    }
                    GatewayCliCommands::Subscribe => {
                        todo!()
                    }
                }
            }
        };

        Ok(())
    }
}
