use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct CnConsumerProcess {
    pub consumer_id: String,
    pub provider_id: Option<String>,
    pub associated_provider: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub state: String,
}
