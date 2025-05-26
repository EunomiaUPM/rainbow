use crate::subscriptions::MicroserviceSubscriptionKey;
use anyhow::bail;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use reqwest::{Client, Error, Response};
use serde_json::json;
use std::time::Duration;
use tracing::{debug, error};
use tracing_subscriber::fmt::format;

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
        let microservice_url = match microservice_key_name {
            MicroserviceSubscriptionKey::Catalog => self.config.get_catalog_host_url().unwrap(),
            MicroserviceSubscriptionKey::ContractNegotiation => {
                self.config.get_contract_negotiation_host_url().unwrap()
            }
            MicroserviceSubscriptionKey::TransferControlPlane => self.config.get_transfer_host_url().unwrap(),
            _ => todo!(),
        };
        let microservice_url = microservice_url.trim_end_matches("/");
        let microservice_tag = match microservice_key_name {
            MicroserviceSubscriptionKey::Catalog => "catalog",
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
