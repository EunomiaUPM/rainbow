use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(flatten)]
    pub foaf: CatalogFoafDeclaration,
    #[serde(flatten)]
    pub dcat: CatalogDcatDeclaration,
    #[serde(flatten)]
    pub dct: CatalogDctDeclaration,
    #[serde(flatten)]
    pub dspace: CatalogDSpaceDeclaration,
    #[serde(rename = "odrl:Offer")]
    pub odrl_offer: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra_fields: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogFoafDeclaration {
    #[serde(rename = "foaf:homepage")]
    pub homepage: String,
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
    pub conforms_to: String,
    #[serde(rename = "dct:creator")]
    pub creator: String,
    #[serde(rename = "dct:identifier")]
    pub identifier: String,
    #[serde(rename = "dct:issued")]
    pub issued: String,
    #[serde(rename = "dct:modified")]
    pub modified: String,
    #[serde(rename = "dct:title")]
    pub title: String,
    #[serde(rename = "dct:description")]
    pub description: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogDSpaceDeclaration {
    #[serde(rename = "dspace:participantId")]
    pub participant_id: String,
}
