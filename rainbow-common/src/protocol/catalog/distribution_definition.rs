use crate::protocol::catalog::dataservice_definition::DataService;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Distribution {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(flatten)]
    pub dcat: DistributionDcatDeclaration,
    #[serde(flatten)]
    pub dct: DistributionDctDeclaration,
    #[serde(rename = "odrl:hasPolicy")]
    pub odrl_offer: serde_json::Value,
    #[serde(rename = "dspace:extraFields")]
    pub extra_fields: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DistributionDcatDeclaration {
    #[serde(rename = "dcat:accessService")]
    pub access_service: Option<DataService>, // Todo should be many to many
} // TODO dcat:format

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DistributionDctDeclaration {
    #[serde(rename = "dct:identifier")]
    pub identifier: String,
    #[serde(rename = "dct:issued")]
    pub issued: chrono::NaiveDateTime,
    #[serde(rename = "dct:modified")]
    pub modified: Option<chrono::NaiveDateTime>,
    #[serde(rename = "dct:title")]
    pub title: Option<String>,
    #[serde(rename = "dct:description")]
    pub description: Vec<String>,
}
