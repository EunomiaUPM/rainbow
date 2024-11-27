use crate::protocol::dataservice_definition::DataService;
use rainbow_db::catalog::entities::distribution;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

impl TryFrom<distribution::Model> for Distribution {
    type Error = anyhow::Error;

    fn try_from(distribution_model: distribution::Model) -> Result<Self, Self::Error> {
        Ok(Distribution {
            context: "https://w3id.org/dspace/2024/1/context.json".to_string(),
            _type: "dcat:Distribution".to_string(),
            id: distribution_model.id.to_string(),
            dcat: DistributionDcatDeclaration {
                access_service: None
            },
            dct: DistributionDctDeclaration {
                identifier: distribution_model.id.to_string(),
                issued: distribution_model.dct_issued,
                modified: distribution_model.dct_modified,
                title: distribution_model.dct_title,
                description: vec![],
            },
            odrl_offer: Value::default(),
            extra_fields: Value::default(),
        })
    }
}
