#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatahubDomain {
    pub urn: String,
    pub properties: DomainProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainProperties {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub search_results: Vec<SearchResult>,
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
    pub domain: DatahubDomain,
    pub glossary_terms: Option<Vec<GlossaryTerm>>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetGraphQLResponse {
    pub data: DatasetSearchResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetSearchResponse {
    pub search_across_entities: DatasetSearchResults,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetSearchResults {
    pub search_results: Vec<DatasetSearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetSearchResult {
    pub entity: DatasetBasicInfo,
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
pub struct Properties {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "customProperties")]
    pub custom_properties: Option<Vec<CustomProperty>>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerWrapper {
    pub owner: Owner,
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
    pub domain: Option<DatahubDomain>,
    pub glossary_terms: Option<GlossaryTerms>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GlossaryTermWrapper {
    pub term: GlossaryTerm,
}


// #[derive(Debug, Deserialize)]
// pub struct AddPolicyRequest {
//     pub property_name: String,
//     pub property_value: String,
// }


#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetGraphQLResponseDetailed {
    pub data: DatasetResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetResponse {
    pub dataset: DatasetEntity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetEntity {
    pub urn: String,
    pub name: String,
    pub platform: Platform,
    pub description: Option<String>,
    pub properties: DatasetProperties,
    pub ownership: Ownership,
    pub tags: Tags,
    pub schema_metadata: Option<SchemaMetadata>,
    pub domain: Domain,
    pub glossary_terms: Option<GlossaryTerms>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetProperties {
    pub name: String,
    pub description: Option<String>,
    pub custom_properties: Option<Vec<CustomProperty>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomProperty {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ownership {
    pub owners: Vec<Owner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub owner: CorpUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorpUser {
    pub username: Option<String>,
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
pub struct Tag {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaMetadata {
    pub fields: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub field_path: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
    pub associated_urn: String,
    pub domain: DomainEntity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainEntity {
    pub urn: String,
    pub properties: DomainProperties,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GlossaryTerms {
    pub terms: Vec<TermWrapper>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TermWrapper {
    pub term: GlossaryTerm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlossaryTerm {
    pub urn: String,
    pub glossaryTermInfo: GlossaryTermInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlossaryTermInfo {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetBasicInfo {
    pub urn: String,
    pub name: String,
}