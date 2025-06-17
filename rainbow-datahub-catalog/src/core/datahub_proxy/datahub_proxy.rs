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

use crate::core::datahub_proxy::datahub_proxy_types::DatasetGraphQLResponseDetailed;
use crate::core::datahub_proxy::datahub_proxy_types::{
    DatahubDataset, DatasetBasicInfo, DatasetGraphQLResponse, DomainProperties, GlossaryTerm, TagProperties};
use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDomain, GraphQLResponse, Platform, Tag};
use crate::core::datahub_proxy::DatahubProxyTrait;
use crate::setup::config::DatahubCatalogApplicationProviderConfig;
use axum::async_trait;
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use reqwest::Client;
use std::time::Duration;
use tracing::debug;

pub struct DatahubProxyService {
    config: DatahubCatalogApplicationProviderConfig,
    client: Client,
}

impl DatahubProxyService {
    pub fn new(config: DatahubCatalogApplicationProviderConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { config, client }
    }
}

#[async_trait]
impl DatahubProxyTrait for DatahubProxyService {
    async fn get_datahub_domains(&self) -> anyhow::Result<Vec<DatahubDomain>> {
        let datahub_host = self.config.get_datahub_host_url().expect("Datahub host not created");
        let datahub_token = self.config.get_datahub_token().expect("Datahub Token not created");
        debug!("{}", datahub_host);
        debug!("{}", datahub_token);
        let graphql_url = format!("{}/api/graphql", datahub_host);
        let query = r#"{
            search(input: { type: DOMAIN, query: "*", start: 0, count: 1000 }) {
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
        let response = self
            .client
            .post(graphql_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", datahub_token))
            .json(&request_body)
            .send()
            .await?;

        let graphql_response: GraphQLResponse<DomainProperties> = response.json().await?;

        let domains = graphql_response
            .data
            .search
            .searchResults
            .into_iter()
            .map(|result| DatahubDomain { urn: result.entity.urn, properties: result.entity.properties })
            .collect();

        Ok(domains)
    }

    async fn get_datahub_tags(&self) -> anyhow::Result<Vec<Tag>> {
    let datahub_host = self.config.get_datahub_host_url().expect("Datahub host not created");
    let datahub_token = self.config.get_datahub_token().expect("Datahub Token not created");
    debug!("{}", datahub_host);
    debug!("{}", datahub_token);
    let graphql_url = format!("{}/api/graphql", datahub_host);
    
    let query = r#"{
    search(input: { type: TAG, query: "*", start: 0, count: 1000 }) {
        searchResults {
            entity {
                ... on Tag {
                    urn
                    properties {
                        name
                        description
                    }
                }
            }
        }
    }
}
"#;

    let request_body = serde_json::json!({
        "query": query
    });

    let response = self
        .client
        .post(graphql_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", datahub_token))
        .json(&request_body)
        .send()
        .await?;

    

    // Mapear a tu estructura Tag
    let graphql_response: GraphQLResponse<TagProperties> = response.json().await?;

    let tags = graphql_response
        .data
        .search
        .searchResults
        .into_iter()
        .map(|result| Tag { 
            urn: result.entity.urn, 
            properties: result.entity.properties 
        })
        .collect();

    Ok(tags)
}

    async fn get_datahub_datasets_by_domain_id(&self, id: String) -> anyhow::Result<Vec<DatahubDataset>> {
        let datahub_host = self.config.get_datahub_host_url().expect("Datahub host not created");
        let datahub_token = self.config.get_datahub_token().expect("Datahub Token not created");
        let graphql_url = format!("{}/api/graphql", datahub_host);
        let query = format!(
            r#"{{
            searchAcrossEntities(input: {{ 
                query: "*", 
                filters: [
                    {{field: "domains", values: ["{}"]}}
                ], 
                types: [DATASET], 
                start: 0, 
                count: 1000 
            }}) {{
                searchResults {{
                    entity {{
                        urn
                        ... on Dataset {{
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
                            schemaMetadata {{
                                fields {{
                                    fieldPath
                                    type
                                }}
                            }}
                            domain {{
                                associatedUrn
                                domain {{
                                    urn
                                    properties {{
                                        name
                                    }}
                                }}
                            }}
                            glossaryTerms {{
                                terms {{
                                    term {{
                                        urn
                                        glossaryTermInfo {{
                                            name
                                            description
                                        }}
                                    }}
                                }}
                            }}
                        }}
                    }}
                }}
            }}
        }}"#,
            id
        );

        let client = reqwest::Client::new();
        let res = client
            .post(&graphql_url)
            .bearer_auth(datahub_token)
            .json(&serde_json::json!({ "query": query }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let mut datasets = Vec::new();
        if let Some(results) = res["data"]["searchAcrossEntities"]["searchResults"].as_array() {
            for result in results {
                if let Some(entity) = result.get("entity") {
                    let urn = entity.get("urn").and_then(|v| v.as_str()).unwrap_or_default().to_string();
                    let name = entity.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
                    let platform = entity.get("platform")
                        .cloned()
                        .and_then(|p| serde_json::from_value(p).ok())
                        .unwrap_or_else(|| Platform { name: "".to_string() });
                    let description = entity.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());

                    let tag_names = entity
                        .get("tags")
                        .and_then(|tags| tags.get("tags"))
                        .and_then(|tags| tags.as_array())
                        .map(|tags| {
                            tags.iter()
                                .filter_map(|tw| tw.get("tag").and_then(|tag| tag.get("name")).and_then(|n| n.as_str()).map(|s| s.to_string()))
                                .collect::<Vec<String>>()
                        })
                        .unwrap_or_default();

                    let custom_props: Vec<(String, String)> = entity
                        .get("properties")
                        .and_then(|p| p.get("customProperties"))
                        .and_then(|cp| cp.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|cp| {
                                    let key = cp.get("key")?.as_str()?;
                                    let value = cp.get("value")?.as_str()?;
                                    Some((key.to_string(), value.to_string()))
                                })
                                .collect()
                        })
                        .unwrap_or_default();

                    let domain = entity
                        .get("domain")
                        .and_then(|d| d.get("domain"))
                        .map(|d| DatahubDomain {
                            urn: d.get("urn").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                            properties: DomainProperties {
                                name: d.get("properties").and_then(|p| p.get("name")).and_then(|n| n.as_str()).unwrap_or_default().to_string(),
                                description: None,
                            },
                        })
                        .unwrap_or(DatahubDomain {
                            urn: "".to_string(),
                            properties: DomainProperties {
                                name: "".to_string(),
                                description: None,
                            },
                        });

                    let glossary_terms = entity
                        .get("glossaryTerms")
                        .and_then(|gt| gt.get("terms"))
                        .and_then(|terms| terms.as_array())
                        .map(|terms| {
                            terms
                                .iter()
                                .filter_map(|t| t.get("term"))
                                .filter_map(|term| serde_json::from_value::<GlossaryTerm>(term.clone()).ok())
                                .collect::<Vec<GlossaryTerm>>()
                        });

                    let dataset = DatahubDataset {
                        urn,
                        name,
                        platform,
                        description,
                        tag_names,
                        custom_properties: custom_props,
                        domain,
                        glossary_terms,
                    };

                    datasets.push(dataset);
                }
            }
        }

        Ok(datasets)
    }
    
    async fn get_datahub_dataset_by_id(&self, id: String) -> anyhow::Result<DatahubDataset> {
        let datahub_host = self.config.get_datahub_host_url().expect("Datahub host not created");
        let datahub_token = self.config.get_datahub_token().expect("Datahub Token not created");
        let graphql_url = format!("{}/api/graphql", datahub_host);
        let query = format!(
            r#"{{
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
                schemaMetadata {{
                    fields {{
                        fieldPath
                        type
                    }}
                }}
                domain {{
                    associatedUrn
                    domain {{
                        urn
                        properties {{
                            name
                        }}
                    }}
                }}
                glossaryTerms {{
                    terms {{
                        term {{
                            urn
                            glossaryTermInfo {{
                                name
                                description
                            }}
                        }}
                    }}
                }}
            }}
        }}"#,
            id
        );

        let request_body = serde_json::json!({
            "query": query
        });

        let response = self
            .client
            .post(graphql_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", datahub_token))
            .json(&request_body)
            .send()
            .await?;

        let graphql_response: DatasetGraphQLResponseDetailed = response.json().await?;

        let dataset = graphql_response.data.dataset;

        let custom_props: Vec<(String, String)> = dataset.properties.customProperties
            .unwrap_or_default()
            .into_iter()
            .map(|cp| (cp.key, cp.value))
            .collect();

        let domain = DatahubDomain {
            urn: dataset.domain.domain.urn,
            properties: DomainProperties {
                name: dataset.domain.domain.properties.name,
                description: None,
            },
        };

        let glossary_terms = dataset.glossaryTerms.map(|gt| {
            gt.terms
                .into_iter()
                .map(|t| t.term)
                .collect::<Vec<GlossaryTerm>>()
        });

        let dataset = DatahubDataset {
            urn: dataset.urn,
            name: dataset.name,
            platform: dataset.platform,
            description: dataset.description,
            tag_names: dataset.tags.tags.into_iter().map(|tw| tw.tag.properties.name).collect(),
            custom_properties: custom_props,
            domain,
            glossary_terms,
        };

        
        Ok(dataset)
    }
}
