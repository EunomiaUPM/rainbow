use reqwest::header::HeaderMap;
use reqwest::Body;
use std::collections::HashMap;
use urn::Urn;


pub struct TransferEventKafkaPayload {
    pub key: Option<Vec<u8>>,
    pub payload: Vec<u8>,
    pub topic: String,
    pub partition: Option<i32>,
    pub offset: Option<i64>,
    pub headers: Option<HashMap<String, Vec<u8>>>,
}

pub enum TransferEventPayloadTypes {
    HTTP(HeaderMap, Body),
    Kafka(TransferEventKafkaPayload),
    NiFi,
}

pub struct TransferEvent {
    pub transfer_event_id: Urn,
    pub payload: TransferEventPayloadTypes,
    pub created_at: chrono::DateTime<chrono::Utc>,
}