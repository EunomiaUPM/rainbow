use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    #[serde(rename="@type")]
    pub _type: String,
    #[serde(flatten)]
    pub dcat: DatasetDcatDeclaration,
    #[serde(flatten)]
    pub dct: DatasetDctDeclaration,
    #[serde(rename="odrl:Offer")]
    pub odrl_offer: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra_fields: Vec<serde_json::Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetDcatDeclaration {
    #[serde(rename="dcat:theme")]
    pub theme: String,
    #[serde(rename="dcat:keyword")]
    pub keyword: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetDctDeclaration {
    #[serde(rename="dct:conformsTo")]
    pub conforms_to: String,
    #[serde(rename="dct:creator")]
    pub creator: String,
    #[serde(rename="dct:identifier")]
    pub identifier: String,
    #[serde(rename="dct:issued")]
    pub issued: String,
    #[serde(rename="dct:modified")]
    pub modified: String,
    #[serde(rename="dct:title")]
    pub title: String,
    #[serde(rename="dct:description")]
    pub description: Vec<String>
}
