use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "protocol")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProtocolSpec {
    Http(HttpSpec),
    Kafka(KafkaSpec),
    // Ftp(FtpSpec),
    // ... S3, AzureBlob, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpSpec {
    pub url_template: String, // Soporta {{VARS}}
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body_template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaSpec {
    pub brokers: Vec<String>,
    pub topic: String,
    pub group_id: Option<String>,
}
