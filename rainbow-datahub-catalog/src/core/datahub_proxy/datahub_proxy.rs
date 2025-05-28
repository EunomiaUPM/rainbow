// use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDataset, DatahubDomain};
use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDomain, DomainProperties, GraphQLResponse, SearchResponse, SearchResults, SearchResult, Entity};
use crate::core::datahub_proxy::DatahubProxyTrait;
use axum::async_trait;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
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
        // URL específica de DataHub
        // let graphql_url = "http://datahub-gms-federado:8080/api/graphql";
        // let dh_host = self.config.get_datahub_host_url();
        // let graphql_url = "http://192.168.64.29:8080/api/graphql";
        let graphql_url = "http://localhost:8084/api/graphql";
        let query = r#"{
            search(input: { type: DOMAIN, query: "*", start: 0, count: 50 }) {
                searchResults {
                    entity {
                        urn
                        ... on Domain {
                            properties {
                                name
                                description
                            }
                        }
                    }
                }
            }
        }"#;

        let request_body = serde_json::json!({
            "query": query
        });

        // Aquí debes poner tu token de autorización
        let token = "eyJhbGciOiJIUzI1NiJ9.eyJhY3RvclR5cGUiOiJVU0VSIiwiYWN0b3JJZCI6ImRydWdzQGRydWdzLmNvbSIsInR5cGUiOiJQRVJTT05BTCIsInZlcnNpb24iOiIyIiwianRpIjoiNGEzOTExYTgtNWYxYS00OWE4LWI4MTEtMDU4ZDMyOTgwYjZiIiwic3ViIjoiZHJ1Z3NAZHJ1Z3MuY29tIiwiZXhwIjoxNzUxMDE0NDI1LCJpc3MiOiJkYXRhaHViLW1ldGFkYXRhLXNlcnZpY2UifQ.-S7uV_rCesUQ92bse8TzaaeZX_WFsAKc3kh3YsWcvxo";

        let response = self.client
            .post(graphql_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .json(&request_body)
            .send()
            .await?;

        let graphql_response: GraphQLResponse = response.json().await?;

        let domains = graphql_response
            .data
            .search
            .searchResults
            .into_iter()
            .map(|result| DatahubDomain {
                urn: result.entity.urn,
                properties: result.entity.properties,
            })
            .collect();

        Ok(domains)
    }

    // async fn get_datahub_domain_by_id(&self, id: String) -> anyhow::Result<DatahubDomain> {
    //     todo!()
    // }

    // async fn get_datahub_datasets_by_domain_id(&self, id: String) -> anyhow::Result<Vec<DatahubDataset>> {
    //     todo!()
    // }

    // async fn get_datahub_dataset_by_id(&self, id: String) -> anyhow::Result<DatahubDataset> {
    //     todo!()
    // }
}