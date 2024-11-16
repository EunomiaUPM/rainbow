use crate::data::entities::catalog::Model as CatalogModel;
use crate::protocol::dataservice_definition::DataService;
use crate::protocol::dataset_definition::Dataset;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

impl TryFrom<CatalogModel> for Catalog {
    type Error = ();

    fn try_from(catalog_model: CatalogModel) -> anyhow::Result<Self, Self::Error> {
        let catalog_out = Catalog {
            context: "https://w3id.org/dspace/2024/1/context.json".to_string(),
            _type: "dcat:Catalog".to_string(),
            id: catalog_model.id.to_string(),
            foaf: CatalogFoafDeclaration {
                homepage: catalog_model.foaf_home_page
            },
            dcat: CatalogDcatDeclaration { // Array of strings...
                theme: "".to_string(),
                keyword: "".to_string(),
            },
            dct: CatalogDctDeclaration {
                conforms_to: catalog_model.dct_conforms_to,
                creator: catalog_model.dct_creator,
                identifier: catalog_model.id.to_string(),
                issued: catalog_model.dct_issued,
                modified: catalog_model.dct_modified,
                title: catalog_model.dct_title,
                description: vec![],
            },
            dspace: CatalogDSpaceDeclaration {
                participant_id: catalog_model.dspace_participant_id
            },
            odrl_offer: Value::default(),
            extra_fields: Value::default(),
            datasets: vec![],
            data_services: vec![],
        };

        Ok(catalog_out)
    }
}
