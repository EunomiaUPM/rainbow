use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct TransferConsumerProcess {
    pub id: String,
    pub consumer_pid: String,
    pub provider_pid: Option<String>,
    pub associated_provider: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub data_plane_id: Option<String>,
    pub data_address: Option<serde_json::Value>,
    pub restart_flag: bool,
    pub state: String,
}
