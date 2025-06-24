use crate::gateway::core::business::BusinessCatalogTrait;
use anyhow::{anyhow, bail};
use axum::async_trait;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use rainbow_common::protocol::datahub_proxy::datahub_proxy_types::{DatahubDataset, DatahubDomain};
use rainbow_db::datahub::entities::policy_templates;
use reqwest::Client;
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
    async fn get_catalogs(&self, token: String) -> anyhow::Result<Vec<DatahubDomain>> {
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

    async fn get_datasets_by_catalog(&self, catalog_id: Urn, token: String) -> anyhow::Result<Vec<DatahubDataset>> {
        let base_url = self.config.get_catalog_host_url().unwrap();
        let url = format!("{}/api/v1/datahub/domains/{}/datasets", base_url, catalog_id);
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch catalogs");
        }
        let res =
            req.json::<Vec<DatahubDataset>>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn get_dataset(&self, dataset_id: Urn, token: String) -> anyhow::Result<DatahubDataset> {
        let base_url = self.config.get_catalog_host_url().unwrap();
        let url = format!("{}/api/v1/datahub/domains/datasets/{}", base_url, dataset_id);
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch catalogs");
        }
        let res =
            req.json::<DatahubDataset>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn get_policy_templates(&self, token: String) -> anyhow::Result<Vec<policy_templates::Model>> {
        let base_url = self.config.get_catalog_host_url().unwrap();
        let url = format!("{}/api/v1/datahub/policy-templates", base_url);
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch policy-templates");
        }
        let res =
            req.json::<Vec<policy_templates::Model>>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn get_policy_template_by_id(&self, template_id: String, token: String) -> anyhow::Result<policy_templates::Model> {
        let base_url = self.config.get_catalog_host_url().unwrap();
        let url = format!("{}/api/v1/datahub/policy-templates/{}", base_url, template_id);
        let req = self.client.get(url).send().await.map_err(|e| anyhow!("lol {}", e.to_string()))?;
        if req.status().is_success() == false {
            bail!("not able to fetch policy-templates");
        }
        let res =
            req.json::<policy_templates::Model>().await.map_err(|e| anyhow!("not deserializable, {}", e.to_string()))?;
        Ok(res)
    }

    async fn get_policy_offers_by_dataset(&self, dataset_id: Urn, token: String) -> anyhow::Result<Vec<OdrlOffer>> {
        todo!()
    }

    async fn post_policy_offer(
        &self,
        dataset_id: Urn,
        odrl_offer: OdrlOffer,
        token: String,
    ) -> anyhow::Result<OdrlOffer> {
        todo!()
    }

    async fn delete_policy_offer(&self, dataset_id: Urn, policy_id: Urn, token: String) -> anyhow::Result<()> {
        todo!()
    }
}
