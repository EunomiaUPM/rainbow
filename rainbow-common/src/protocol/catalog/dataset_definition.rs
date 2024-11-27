use super::distribution_definition::Distribution;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dataset {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(flatten)]
    pub dcat: DatasetDcatDeclaration,
    #[serde(flatten)]
    pub dct: DatasetDctDeclaration,
    #[serde(rename = "odrl:hasPolicy")]
    pub odrl_offer: serde_json::Value,
    #[serde(rename = "odrl:extraFields")]
    pub extra_fields: serde_json::Value,
    #[serde(rename = "dcat:distribution")]
    pub distribution: Vec<Distribution>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatasetDcatDeclaration {
    #[serde(rename = "dcat:theme")]
    pub theme: String,
    #[serde(rename = "dcat:keyword")]
    pub keyword: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatasetDctDeclaration {
    #[serde(rename = "dct:conformsTo")]
    pub conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    pub creator: Option<String>,
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

