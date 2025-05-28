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