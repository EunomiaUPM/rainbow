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
use crate::gateway::core::business::business::BusinessServiceForDatahub;
use crate::gateway::http::business_router::RainbowBusinessRouter;
use axum::serve;
use clap::{Parser, Subcommand};
use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Business Gateway Server")]
#[command(version = "0.2")]
struct GatewayCli {
    #[command(subcommand)]
    command: GatewayCliCommands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum GatewayCliCommands {
    Start,
    Subscribe,
    Build,
}

pub struct GatewayCommands;

impl GatewayCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = GatewayCli::parse();

        // run scripts
        match cli.command {
            GatewayCliCommands::Start => {
                let config = ApplicationProviderConfig::default();
                let config = config.merge_dotenv_configuration();
                let table = json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                info!("Current config:\n{}", table);

                let gateway_service = Arc::new(BusinessServiceForDatahub::new(config.clone()));
                let gateway_router = RainbowBusinessRouter::new(gateway_service).router();
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
                debug!("Subscribe to provider")
            }
            GatewayCliCommands::Build => {
                debug!("Subscribe to build fe into app")
            }
        }
        Ok(())
    }
}
