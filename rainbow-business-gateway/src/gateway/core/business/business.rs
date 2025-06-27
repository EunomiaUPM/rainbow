use crate::gateway::core::business::BusinessCatalogTrait;
use crate::gateway::http::business_router_types::RainbowBusinessNegotiationRequest;
use anyhow::{anyhow, bail};
use axum::async_trait;
use rainbow_common::auth::business::RainbowBusinessLoginRequest;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use rainbow_common::mates::Mates;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_odrl::{OdrlOffer, OdrlPolicyInfo};
use rainbow_common::protocol::datahub_proxy::datahub_proxy_types::{DatahubDataset, DatahubDomain};
use rainbow_db::datahub::entities::policy_templates;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;
use urn::Urn;

pub struct BusinessServiceForDatahub {
    client: Client,
    config: ApplicationProviderConfig,
}

impl BusinessServiceForDatahub {
    pub fn new(config: ApplicationProviderConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { client, config }
    }
}
#[async_trait]
impl BusinessCatalogTrait for BusinessServiceForDatahub {
    async fn get_catalogs(&self, _token: String) -> anyhow::Result<Vec<DatahubDomain>> {
        let base_url = self.config.get_catalog_host_url().unwrap();
        let url = format!("{}/api/v1/datahub/domains", base_url);
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch catalogs");
        }
        let res =
            req.json::<Vec<DatahubDomain>>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn get_datasets_by_catalog(&self, catalog_id: Urn, _token: String) -> anyhow::Result<Vec<DatahubDataset>> {
        let base_url = self.config.get_catalog_host_url().unwrap();
        let url = format!(
            "{}/api/v1/datahub/domains/{}/datasets",
            base_url, catalog_id
        );
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch catalogs");
        }
        let res =
            req.json::<Vec<DatahubDataset>>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn get_dataset(&self, dataset_id: Urn, _token: String) -> anyhow::Result<DatahubDataset> {
        let base_url = self.config.get_catalog_host_url().unwrap();
        let url = format!(
            "{}/api/v1/datahub/domains/datasets/{}",
            base_url, dataset_id
        );
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch catalogs");
        }
        let res = req.json::<DatahubDataset>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn get_policy_templates(&self, _token: String) -> anyhow::Result<Vec<policy_templates::Model>> {
        let base_url = self.config.get_catalog_host_url().unwrap();
        let url = format!("{}/api/v1/datahub/policy-templates", base_url);
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch policy-templates");
        }
        let res = req
            .json::<Vec<policy_templates::Model>>()
            .await
            .map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn get_policy_template_by_id(
        &self,
        template_id: String,
        _token: String,
    ) -> anyhow::Result<policy_templates::Model> {
        let base_url = self.config.get_catalog_host_url().unwrap();
        let url = format!(
            "{}/api/v1/datahub/policy-templates/{}",
            base_url, template_id
        );
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch policy-templates");
        }
        let res = req
            .json::<policy_templates::Model>()
            .await
            .map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn get_policy_offers_by_dataset(&self, dataset_id: Urn, _token: String) -> anyhow::Result<Vec<OdrlOffer>> {
        let base_url = self.config.get_catalog_host_url().unwrap();
        let url = format!("{}/api/v1/datasets/{}/policies", base_url, dataset_id);
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch policy-templates");
        }
        let res = req.json::<Vec<OdrlOffer>>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn post_policy_offer(
        &self,
        dataset_id: Urn,
        odrl_offer: OdrlPolicyInfo,
        _token: String,
    ) -> anyhow::Result<OdrlOffer> {
        let base_url = self.config.get_catalog_host_url().unwrap();
        let url = format!("{}/api/v1/datasets/{}/policies", base_url, dataset_id);
        let req = self.client.post(url).json(&odrl_offer).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch policy-templates");
        }
        let res = req.json::<OdrlOffer>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn delete_policy_offer(&self, dataset_id: Urn, policy_id: Urn, _token: String) -> anyhow::Result<()> {
        let base_url = self.config.get_catalog_host_url().unwrap();
        let url = format!("{}/api/v1/datasets/{}/policies", base_url, dataset_id);
        let req = self.client.delete(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch policy-templates");
        }
        Ok(())
    }

    async fn get_business_negotiation_requests(&self, _token: String) -> anyhow::Result<Vec<ContractAckMessage>> {
        let base_url = self.config.get_contract_negotiation_host_url().unwrap();
        let url = format!("{}/api/v1/contract-negotiation/processes", base_url);
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch contract negotiation processes");
        }
        let res = req
            .json::<Vec<ContractAckMessage>>()
            .await
            .map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn get_business_negotiation_request_by_id(
        &self,
        request_id: Urn,
        _token: String,
    ) -> anyhow::Result<ContractAckMessage> {
        let base_url = self.config.get_contract_negotiation_host_url().unwrap();
        let url = format!(
            "{}/api/v1/contract-negotiation/processes/{}",
            base_url, request_id
        );
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch contract negotiation process");
        }
        let res =
            req.json::<ContractAckMessage>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn get_consumer_negotiation_requests(&self, _token: String) -> anyhow::Result<Vec<ContractAckMessage>> {
        let base_url = self.config.get_contract_negotiation_host_url().unwrap();
        let url = format!("{}/api/v1/contract-negotiation/processes", base_url);
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch contract negotiation processes");
        }
        let res = req
            .json::<Vec<ContractAckMessage>>()
            .await
            .map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn get_consumer_negotiation_request_by_id(
        &self,
        request_id: Urn,
        _token: String,
    ) -> anyhow::Result<ContractAckMessage> {
        let base_url = self.config.get_contract_negotiation_host_url().unwrap();
        let url = format!(
            "{}/api/v1/contract-negotiation/processes/{}",
            base_url, request_id
        );
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch contract negotiation process");
        }
        let res =
            req.json::<ContractAckMessage>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn accept_request(&self, request_id: Urn, _token: String) -> anyhow::Result<()> {
        // get cn request, use
        Ok(())
    }

    async fn create_request(&self, input: RainbowBusinessNegotiationRequest, _token: String) -> anyhow::Result<Value> {
        // fetch base url for consumer and its token
        // TODO replace bypassing by Rodrigo's implementation
        let base_url = self.config.get_contract_negotiation_host_url().unwrap();
        let url = format!(
            "{}/api/v1/mates/{}",
            base_url, input.consumer_participant_id
        );
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch consumer user");
        }
        let consumer_participant =
            req.json::<Mates>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;

        // fetch base url for provider
        let base_url = self.config.get_contract_negotiation_host_url().unwrap();
        let url = format!("{}/api/v1/mates/me", base_url);
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch provider user");
        }
        let provider_participant =
            req.json::<Mates>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;

        // fetch providerParticipantId by consumer
        // first we get all data from consumer
        let base_url = self.config.get_contract_negotiation_host_url().unwrap();
        let url = format!(
            "{}/api/v1/mates/bypass/{}",
            base_url,
            consumer_participant.participant_id.clone()
        );
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch provider user");
        }
        let consumer_participants_bypassed =
            req.json::<Vec<Mates>>().await.map_err(|e| anyhow!("not deserializable in mates, {}", e.to_string()))?;
        let a =
            consumer_participants_bypassed.iter().find(|p| p.participant_type == "Provider").expect("aaa").to_owned();

        // create SetupContractNegotiationRequest message
        let setup_request_message = json!({
            "providerParticipantId": a.participant_id,
            "offer": input.offer.clone()
        });

        // RPC to consumer with message
        let base_url = consumer_participant.base_url.unwrap_or_default();
        let url = format!("{}/api/v1/negotiations/rpc/setup-request", base_url);
        let req = self
            .client
            .post(url)
            .json(&setup_request_message)
            .header(
                "Authorization",
                format!("Bearer {}", consumer_participant.token.unwrap_or_default()),
            )
            .send()
            .await
            .map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to start contract negotiation from consumer");
        }
        let res = req.json().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;

        Ok(res)
    }

    async fn login(&self, input: RainbowBusinessLoginRequest) -> anyhow::Result<String> {
        let base_url = self.config.get_contract_negotiation_host_url().unwrap();
        let url = format!("{}/api/v1/generate/uri", base_url);
        let req = self.client.post(url).json(&input).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to login in provider");
        }
        let res = req.text().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn login_poll(&self, input: RainbowBusinessLoginRequest) -> anyhow::Result<Value> {
        let base_url = self.config.get_contract_negotiation_host_url().unwrap();
        let url = format!("{}/api/v1/busmates/token", base_url);
        let req = self.client.post(url).json(&input).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;

        if req.status().is_client_error() || req.status().is_server_error() {
            bail!("not able to poll login in provider");
        }
        if req.status().is_informational() {
            bail!("user still using the wallet");
        }

        let res = req.json::<Value>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }
}
