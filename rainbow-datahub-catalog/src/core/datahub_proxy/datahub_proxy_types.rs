use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatahubDomain;
#[derive(Debug, Serialize, Deserialize)]
pub struct DatahubDataset;

#[derive(Debug, Deserialize)]
pub struct DomainsQueryOptions {
    pub a: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DatasetsQueryOptions {
    pub a: Option<String>,
}