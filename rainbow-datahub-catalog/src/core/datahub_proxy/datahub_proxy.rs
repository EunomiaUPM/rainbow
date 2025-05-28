use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDataset, DatahubDomain};
use crate::core::datahub_proxy::DatahubProxyTrait;
use axum::async_trait;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use reqwest::Client;
use std::time::Duration;

pub struct DatahubProxyService {
    config: ApplicationProviderConfig,
    client: Client,
}

impl DatahubProxyService {
    pub fn new(config: ApplicationProviderConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self {
            config,
            client,
        }
    }
}

#[async_trait]
impl DatahubProxyTrait for DatahubProxyService {
    async fn get_datahub_domains(&self) -> anyhow::Result<Vec<DatahubDomain>> {
        todo!()
    }

    async fn get_datahub_domain_by_id(&self, id: String) -> anyhow::Result<DatahubDomain> {
        todo!()
    }

    async fn get_datahub_datasets_by_domain_id(&self, id: String) -> anyhow::Result<Vec<DatahubDataset>> {
        todo!()
    }

    async fn get_datahub_dataset_by_id(&self, id: String) -> anyhow::Result<DatahubDataset> {
        todo!()
    }
}