use crate::protocol::catalog::dataservice_definition::DataService;
use crate::protocol::catalog::dataset_definition::Dataset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(flatten)]
    pub foaf: CatalogFoafDeclaration,
    #[serde(flatten)]
    pub dcat: CatalogDcatDeclaration,
    #[serde(flatten)]
    pub dct: CatalogDctDeclaration,
    #[serde(flatten)]
    pub dspace: CatalogDSpaceDeclaration,
    #[serde(rename = "odrl:hasPolicy")]
    pub odrl_offer: serde_json::Value,
    #[serde(rename = "dspace:extraFields")]
    pub extra_fields: serde_json::Value,
    #[serde(rename = "dcat:dataset")]
    pub datasets: Vec<Dataset>,
    #[serde(rename = "dcat:service")]
    pub data_services: Vec<DataService>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogFoafDeclaration {
    #[serde(rename = "foaf:homepage")]
    pub homepage: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogDcatDeclaration {
    #[serde(rename = "dcat:theme")]
    pub theme: String,
    #[serde(rename = "dcat:keyword")]
    pub keyword: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogDctDeclaration {
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
    pub description: Vec<String>, // TODO set descriptions in all...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogDSpaceDeclaration {
    #[serde(rename = "dspace:participantId")]
    pub participant_id: Option<String>,
}
