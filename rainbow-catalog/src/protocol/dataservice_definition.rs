use crate::data::entities::dataservice::Model as DataserviceModel;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataService {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(flatten)]
    pub dcat: DataServiceDcatDeclaration,
    #[serde(flatten)]
    pub dct: DataServiceDctDeclaration,
    #[serde(rename = "odrl:hasPolicy")]
    pub odrl_offer: serde_json::Value,
    #[serde(rename = "dspace:extraFields")]
    pub extra_fields: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataServiceDcatDeclaration {
    #[serde(rename = "dcat:theme")]
    pub theme: String,
    #[serde(rename = "dcat:keyword")]
    pub keyword: String,
    #[serde(rename = "dcat:endpointDescription")]
    pub endpoint_description: String,
    #[serde(rename = "dcat:endpointURL")]
    pub endpoint_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataServiceDctDeclaration {
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

impl TryFrom<DataserviceModel> for DataService {
    type Error = anyhow::Error;

    fn try_from(dataservice_model: DataserviceModel) -> Result<Self, Self::Error> {
        Ok(DataService {
            context: "https://w3id.org/dspace/2024/1/context.json".to_string(),
            _type: "dcat:DataService".to_string(),
            id: dataservice_model.id.to_string(),
            dcat: DataServiceDcatDeclaration {
                theme: "".to_string(),
                keyword: "".to_string(),
                endpoint_description: dataservice_model.dcat_endpoint_description.unwrap_or("".to_string()),
                endpoint_url: dataservice_model.dcat_endpoint_url,
            },
            dct: DataServiceDctDeclaration {
                conforms_to: dataservice_model.dct_conforms_to,
                creator: dataservice_model.dct_creator,
                identifier: dataservice_model.dct_identifier.unwrap_or_else(|| dataservice_model.id.to_string()),
                issued: dataservice_model.dct_issued,
                modified: dataservice_model.dct_modified,
                title: dataservice_model.dct_title,
                description: vec![],
            },
            odrl_offer: Value::default(),
            extra_fields: Value::default(),
        })
    }
}