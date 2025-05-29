use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct DatahubDomain;
// #[derive(Debug, Serialize, Deserialize)]
// pub struct DatahubDataset;

// #[derive(Debug, Deserialize)]
// pub struct DomainsQueryOptions {
//     pub a: Option<String>,
// }

// #[derive(Debug, Deserialize)]
// pub struct DatasetsQueryOptions {
//     pub a: Option<String>,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct DatahubDomain {
    pub urn: String,
    pub properties: DomainProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainProperties {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainsQueryOptions {
    pub start: Option<i32>,
    pub count: Option<i32>,
    pub query: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub search: SearchResults,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLResponse {
    pub data: SearchResponse,
    pub extensions: serde_json::Value,  // Para el campo "extensions" que está vacío
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResults {
    pub searchResults: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub entity: Entity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub urn: String,
    pub properties: DomainProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatahubDataset {
    pub urn: String,
    pub name: String,
    pub platform: Platform,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetGraphQLResponse {
    pub data: DatasetSearchResponse,
    pub extensions: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetSearchResponse {
    pub searchAcrossEntities: DatasetSearchResults,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetSearchResults {
    pub searchResults: Vec<DatasetSearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetSearchResult {
    pub entity: DatasetEntity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetEntity {
    pub urn: String,
    pub name: String,
    pub platform: Platform,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetsQueryOptions {
    pub start: Option<i32>,
    pub count: Option<i32>,
    pub query: Option<String>,
    pub domain_id: Option<String>,
}
