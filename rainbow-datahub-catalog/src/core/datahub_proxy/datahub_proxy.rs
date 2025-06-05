// use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDataset, DatahubDomain};
use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDomain, DomainProperties, GraphQLResponse, SearchResponse, SearchResults, SearchResult, Entity};
use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDataset, Platform, DatasetGraphQLResponse, DatasetSearchResponse, DatasetSearchResults, DatasetSearchResult, DatasetEntity};
use crate::core::datahub_proxy::datahub_proxy_types::{DatasetGraphQLResponseDetailed};
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
        let graphql_url = "http://localhost:8086/api/graphql";
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
         
        let token = "eyJhbGciOiJIUzI1NiJ9.eyJhY3RvclR5cGUiOiJVU0VSIiwiYWN0b3JJZCI6ImRydWdzQGRydWdzLmNvbSIsInR5cGUiOiJQRVJTT05BTCIsInZlcnNpb24iOiIyIiwianRpIjoiYmZkMTA5MjYtODE0MC00ODk1LTliNTgtNTMzMWMxMjY2MWMwIiwic3ViIjoiZHJ1Z3NAZHJ1Z3MuY29tIiwiZXhwIjoxNzUxNTM3NzQxLCJpc3MiOiJkYXRhaHViLW1ldGFkYXRhLXNlcnZpY2UifQ.VvTiXmU98Hnurdg9g_xINtz3zyvtM2SpF6Ad23h7kJM";

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

    async fn get_datahub_datasets_by_domain_id(&self, id: String) -> anyhow::Result<Vec<DatahubDataset>> {
       let graphql_url = "http://localhost:8086/api/graphql";
        let query = format!(r#"{{
            searchAcrossEntities(input: {{ 
                query: "*", 
                filters: [
                    {{field: "domains", values: ["{}"]}}
                ], 
                types: [DATASET], 
                start: 0, 
                count: 50 
            }}) {{
                searchResults {{
                    entity {{
                        urn
                        ... on Dataset {{
                            name
                            platform {{
                                name
                            }}
                        }}
                    }}
                }}
            }}
        }}"#, id);

        let request_body = serde_json::json!({
            "query": query
        });

        let token = "eyJhbGciOiJIUzI1NiJ9.eyJhY3RvclR5cGUiOiJVU0VSIiwiYWN0b3JJZCI6ImRydWdzQGRydWdzLmNvbSIsInR5cGUiOiJQRVJTT05BTCIsInZlcnNpb24iOiIyIiwianRpIjoiYmZkMTA5MjYtODE0MC00ODk1LTliNTgtNTMzMWMxMjY2MWMwIiwic3ViIjoiZHJ1Z3NAZHJ1Z3MuY29tIiwiZXhwIjoxNzUxNTM3NzQxLCJpc3MiOiJkYXRhaHViLW1ldGFkYXRhLXNlcnZpY2UifQ.VvTiXmU98Hnurdg9g_xINtz3zyvtM2SpF6Ad23h7kJM";

        let response = self.client
            .post(graphql_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .json(&request_body)
            .send()
            .await?;

        let graphql_response: DatasetGraphQLResponse = response.json().await?;

        let datasets = graphql_response
            .data
            .searchAcrossEntities
            .searchResults
            .into_iter()
            .map(|result| DatahubDataset {
                urn: result.entity.urn,
                name: result.entity.name,
                platform: result.entity.platform,
                description: None,
                tag_names: vec![],
                custom_properties: vec![],
            })
            .collect();

        Ok(datasets)
    }
    async fn get_datahub_dataset_by_id(&self, id: String) -> anyhow::Result<DatahubDataset> {
        let graphql_url = "http://localhost:8086/api/graphql";
        let query = format!(r#"{{
        dataset(urn: "{}") {{
            urn
            name
            platform {{ name }}
            description
            properties {{
                name
                description
                customProperties {{
                    key
                    value
                }}
            }}
            ownership {{
                owners {{
                    owner {{
                        ... on CorpUser {{
                            username
                        }}
                    }}
                }}
            }}
            tags {{
                tags {{
                    tag {{
                        name
                    }}
                }}
            }}
        }}
    }}"#, id);

        let request_body = serde_json::json!({
        "query": query
    });

        let token = "eyJhbGciOiJIUzI1NiJ9.eyJhY3RvclR5cGUiOiJVU0VSIiwiYWN0b3JJZCI6ImRydWdzQGRydWdzLmNvbSIsInR5cGUiOiJQRVJTT05BTCIsInZlcnNpb24iOiIyIiwianRpIjoiYmZkMTA5MjYtODE0MC00ODk1LTliNTgtNTMzMWMxMjY2MWMwIiwic3ViIjoiZHJ1Z3NAZHJ1Z3MuY29tIiwiZXhwIjoxNzUxNTM3NzQxLCJpc3MiOiJkYXRhaHViLW1ldGFkYXRhLXNlcnZpY2UifQ.VvTiXmU98Hnurdg9g_xINtz3zyvtM2SpF6Ad23h7kJM";

        let response = self.client
            .post(graphql_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .json(&request_body)
            .send()
            .await?;

        let graphql_response: DatasetGraphQLResponseDetailed = response.json().await?;
        
        let detailed = graphql_response.data.dataset;

        let custom_props = detailed.properties.custom_properties.unwrap_or_default()
            .into_iter()
            .map(|cp| (cp.key, cp.value))
            .collect();

        let dataset = DatahubDataset {
            urn: detailed.urn,
            name: detailed.name,
            platform: detailed.platform,
            description: detailed.description,
            tag_names: detailed.tags.tags.into_iter().map(|tw| tw.tag.name).collect(),
            custom_properties: custom_props,
        };

        Ok(dataset)
    }

    // async fn get_dataset_policies(&self, dataset_urn: &str) -> anyhow::Result<Option<String>> {
    //     let graphql_url = "http://localhost:8086/api/graphql";
    //     let token = "eyJhbGciOiJIUzI1NiJ9.eyJhY3RvclR5cGUiOiJVU0VSIiwiYWN0b3JJZCI6ImRydWdzQGRydWdzLmNvbSIsInR5cGUiOiJQRVJTT05BTCIsInZlcnNpb24iOiIyIiwianRpIjoiNGEzOTExYTgtNWYxYS00OWE4LWI4MTEtMDU4ZDMyOTgwYjZiIiwic3ViIjoiZHJ1Z3NAZHJ1Z3MuY29tIiwiZXhwIjoxNzUxMDE0NDI1LCJpc3MiOiJkYXRhaHViLW1ldGFkYXRhLXNlcnZpY2UifQ.-S7uV_rCesUQ92bse8TzaaeZX_WFsAKc3kh3YsWcvxo";

    //     let query = format!(r#"{{
    //     dataset(urn: "{}") {{
    //         properties {{
    //             customProperties {{
    //                 key
    //                 value
    //             }}
    //         }}
    //     }}
    // }}"#, dataset_urn);

    //     let request_body = serde_json::json!({ "query": query });

    //     let response = self.client
    //         .post(graphql_url)
    //         .header("Content-Type", "application/json")
    //         .header("Authorization", format!("Bearer {}", token))
    //         .json(&request_body)
    //         .send()
    //         .await?;

    //     let graphql_response: DatasetGraphQLResponseDetailed = response.json().await?;

    //     // Buscar la propiedad "policy" dentro de customProperties
    //     if let Some(custom_props) = graphql_response.data.dataset.properties.custom_properties {
    //         for cp in custom_props {
    //             if cp.key == "policy" {
    //                 return Ok(Some(cp.value));
    //             }
    //         }
    //     }

    //     Ok(None)
    // }


    // async fn add_policy_to_dataset(&self, dataset_urn: String, property_name: String, property_value: String) -> anyhow::Result<bool> {
    //     let output = tokio::process::Command::new("python3")
    //         .arg("../scripts/add_policy_to_dataset.py")
    //         .arg(dataset_urn)
    //         .arg(property_name)
    //         .arg(property_value)
    //         .output()
    //         .await?;

    //     Ok(output.status.success())
    // }
}