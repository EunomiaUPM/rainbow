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
    pub description: Option<String>,
    pub tag_names: Vec<String>,
    pub custom_properties: Vec<(String, String)>,
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

/*use serde::Deserialize;*/


#[derive(Debug, Serialize, Deserialize)]
pub struct CustomProperty {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "customProperties")]
    pub custom_properties: Option<Vec<CustomProperty>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "username")]
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ownership {
    pub owners: Vec<OwnerWrapper>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerWrapper {
    pub owner: Owner,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    pub tags: Vec<TagWrapper>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagWrapper {
    pub tag: Tag,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetEntityDetailed {
    pub urn: String,
    pub name: String,
    pub platform: Platform,
    pub description: Option<String>,
    pub properties: Properties,
    pub ownership: Ownership,
    pub tags: Tags,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetResponse {
    pub dataset: DatasetEntityDetailed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetGraphQLResponseDetailed {
    pub data: DatasetResponse,
    pub extensions: serde_json::Value,
}
