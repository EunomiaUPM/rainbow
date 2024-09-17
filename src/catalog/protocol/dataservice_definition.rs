use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataService {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(flatten)]
    pub dcat: DataServiceDcatDeclaration,
    #[serde(flatten)]
    pub dct: DataServiceDctDeclaration,
    #[serde(rename = "odrl:Offer")]
    pub odrl_offer: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra_fields: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataServiceDcatDeclaration {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataServiceDctDeclaration {
    #[serde(rename = "dct:conformsTo")]
    pub conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    pub creator: Option<String>,
    #[serde(rename = "dct:identifier")]
    pub identifier: Option<String>,
    #[serde(rename = "dct:issued")]
    pub issued: String,
    #[serde(rename = "dct:modified")]
    pub modified: String,
    #[serde(rename = "dct:title")]
    pub title: String,
    #[serde(rename = "dct:description")]
    pub description: Vec<String>,
}
