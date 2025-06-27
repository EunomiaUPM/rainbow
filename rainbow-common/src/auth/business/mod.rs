use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RainbowBusinessLoginRequest {
    #[serde(rename = "authRequestId")]
    pub auth_request_id: String,
}