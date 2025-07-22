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

use crate::subscriptions::MicroserviceSubscriptionKey;
use anyhow::bail;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tracing::{debug, error};

pub struct RainbowProviderGatewaySubscriptions {
    config: ApplicationProviderConfig,
    client: Client,
}

impl RainbowProviderGatewaySubscriptions {
    pub fn new(config: ApplicationProviderConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { config, client }
    }
    pub async fn subscribe_to_microservice(&self, microservice_key_name: MicroserviceSubscriptionKey) -> anyhow::Result<()> {
        let is_datahub = self.config.is_datahub_as_catalog();
        let microservice_url = match microservice_key_name {
            MicroserviceSubscriptionKey::Catalog => match is_datahub {
                true => self.config.get_contract_negotiation_host_url().unwrap(),
                false => self.config.get_catalog_host_url().unwrap(),
            }
            MicroserviceSubscriptionKey::ContractNegotiation => {
                self.config.get_contract_negotiation_host_url().unwrap()
            }
            MicroserviceSubscriptionKey::TransferControlPlane => self.config.get_transfer_host_url().unwrap(),
            _ => todo!(),
        };
        let microservice_url = microservice_url.trim_end_matches("/");
        let microservice_tag = match microservice_key_name {
            MicroserviceSubscriptionKey::Catalog => match is_datahub {
                true => "contract-negotiation",
                false => "catalog",
            },
            MicroserviceSubscriptionKey::ContractNegotiation => "contract-negotiation",
            MicroserviceSubscriptionKey::TransferControlPlane => "transfers",
            _ => todo!(),
        };
        let subscription_base = format!("/api/v1/{}/subscriptions", microservice_tag);
        let subscription_url = format!("{}{}", microservice_url, subscription_base);
        debug!(subscription_url);

        let notification_gateway_endpoint = "/incoming-notification";
        let notification_gateway_url = format!(
            "{}{}",
            self.config.get_gateway_host_url().unwrap(),
            notification_gateway_endpoint
        );

        let request = self
            .client
            .post(&subscription_url)
            .json(&json!({
                "callbackAddress": notification_gateway_url
            }))
            .send()
            .await;
        let request = match request {
            Ok(request) => request,
            Err(e) => {
                error!("Error on subscribing. Microservice not available{}", e);
                bail!("Error on subscribing. Microservice not available {}", e)
            }
        };
        if !request.status().is_success() {
            bail!("Error on subscribing. Status {}", request.status());
        }
        Ok(())
    }
}
